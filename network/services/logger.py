import time
import queue
import threading
import logging
import grpc
import os
from network.protos import logger_pb2, logger_pb2_grpc
from network.services import Service

# Service for receiving and processing logs from other hosts
class LoggingServer(Service, logger_pb2_grpc.LoggerServicer):
    # Name used to identify service
    name = "logging_server"
    priority = 2
    
    def __init__(self, output_dir="./output/logging", 
                       consolidated_log_name="consolidated.log",
                       consolidate_on_exit=True):

        self._output_dir = output_dir
        self._consolidated_log = os.path.join(self._output_dir, consolidated_log_name)

        # list of all loggers in format {logger_name : logger, ...}
        self._loggers = {}

        # create output directory
        os.makedirs(self._output_dir, exist_ok=True)

        # message receiving
        self._message_queue = queue.Queue()
        self._running = False
        self._consolidate_on_exit = consolidate_on_exit
        self._logger_thread = None
        self._thread_wait = 1


    # REQUIRED FUNCTIONS

    # Add self to server
    def add_service_to_server(self, server):
        logger_pb2_grpc.add_LoggerServicer_to_server(self, server)


    # gRPC SERVICER FUNCTIONS

    # Called when receiving a new stream of log messages
    def SendLogMessages(self, request_iterator, context):
        for request in request_iterator:
            self._message_queue.put(request)
        return logger_pb2.LogResponse()


    # SERVICE SPECIFIC FUNCTIONS

    def is_running(self):
        return self._running

    # Starts the receiver and processor, spawns threads
    def start(self):
        # only start if not already running
        if not self._running:
            self._log("starting logger service")
            self._running = True
            self._logger_thread = threading.Thread(target=self._run_logger)
            self._logger_thread.start()
        else:
            self._log("logger service already running")
        
    # Stops the receiver and processor, joins threads
    def stop(self, join=True):
        self._log("stop called...")
        # if running, stop and join threads
        if self._running:
            self._log("stopping logger")
            self._running = False
            if join:
                self.join()
        else:
            self._log("not currently running")

    def join(self):
        if self._logger_thread:
            self._logger_thread.join()
            self._logger_thread = None
        else:
            self._log("no processor thread to join")

    # Combine all log entries and sort by timestamp
    def _consolidate_logs(self):
        self._log("consolidating...")

        if not os.path.exists(self._output_dir):
            self._log("no output directory")
            return

        # store all entries in a single list
        all_entries = []

        # loop through each logfile (category) in each host directory
        for filename in os.listdir(self._output_dir):
            logfile = os.path.join(self._output_dir, filename)

            # grab all lines from the file if it not the consolidated log
            if logfile != self._consolidated_log:
                with open(logfile, 'r') as f:
                    all_entries.extend(f)
            else:
                self._log("bad logfile: " + logfile)

        if len(all_entries) > 0:
            # sort by timestamp (first part of string is timestamp, so it works)
            all_entries.sort()

            with open(self._consolidated_log, 'w') as f:
                f.writelines(all_entries)

            self._log(" consolidated " + str(len(all_entries)) + " entries to " + self._consolidated_log)
        else:
            self._log(" nothing to consolidate")

    # checks for new messages and processes them
    def _run_logger(self):
        self._log("logger loop entering")
        while self._running:
            try:
                # look for new messages from the queue
                while not self._message_queue.empty():
                    self._message_log(self._message_queue.get())

                # wait beore trying again
                time.sleep(self._thread_wait)
            except Exception as e:
                self._log("exception in logger: " + str(e))

        if self._consolidate_on_exit:
            self._consolidate_logs()

        self._log("logger loop exiting")
    
    # Log the given message
    # Separates into directories for each host, then loggers (and files) for each category
    def _message_log(self, message):
        try:
            logger_name = message.host

            # Add a new logger if it doesn't exist
            if logger_name not in self._loggers:
                self._log("Adding new logger for "  + logger_name)

                # make new logger
                logger = logging.getLogger(logger_name)
                logger.setLevel(logging.DEBUG)
                fh = logging.FileHandler(os.path.join(self._output_dir, logger_name + ".log"))
                fh.setLevel(logging.DEBUG)
                logger.addHandler(fh)
                self._loggers[logger_name] = logger
            
            # grab the logger
            logger = self._loggers[logger_name]

            # log the message
            logger.debug(self._message_str(message))

        except Exception as e:
            self._log("failed to log: {" + str(message) + "}, Exception: " + str(e))
    
    def _message_str(self, message):
        return " ".join(["[{0}]".format(message.timestamp),
                         "({0})".format(message.host),
                         "<{0}>".format(message.category),
                         "\"{0}\"".format(message.content)])


# Simple service for sending logging messages to some central server
# TODO: Allow dynamic message sending to other servers
class LoggingClient(Service):
    name = "logging_client"
    priority = 2

    # Takes in address of the server to send messages to
    def __init__(self, host="localhost", port=13173, stream_max=10000, stream_max_wait=False):
        self._host = host
        self._port = port
        self._server_address = host + ":" + str(port)
        self._running = False
        self._sender_thread = None
        self._wait_time_sec = 1
        self._stream_max = stream_max  # Prevents one service from hogging
        self._stream_max_wait = stream_max_wait  # If stream_max is reached, wait before sending more
        self._stub = logger_pb2_grpc.LoggerStub(grpc.insecure_channel(self._server_address))
        self._send_function = self._stub.SendLogMessages
        self._message_queue = queue.Queue()


    # Queues a message to be sent to the service
    def log(self, category, content):
        if self._network_manager:
            if self._running:
                message = logger_pb2.LogMessage(timestamp="{0:.10f}".format(time.time()),
                                                   host=self._network_manager.address(),
                                                   category=category,
                                                   content=content)
                self._message_queue.put(message)
            else:
                self._log(" Messaging Client not running")

    def is_running(self):
        return self._running

    # Starts the receiver and processor, spawns thread
    def start(self):
        self._log("Start called on Messaging Client:")
        # only start if not already running
        if not self._running:
            self._log(" Starting Messaging Client")
            self._running = True
            self._sender_thread = threading.Thread(target=self._run_sender)
            self._sender_thread.start()
        else:
            self._log(" Messaging Client already running")

    # Stops the receiver and processor, option join thread
    def stop(self, join=False):
        self._log(" Stop called on Messaging Client")
        # if running, stop and join threads
        if self._running:
            self._log(" Stopping Messaging Client")
            self._running = False

            if join:
                self.join()
        else:
            self._log(" Messaging Client not running")
    
    def join(self):
        if self._sender_thread:
            self._sender_thread.join()
            self._sender_thread = None
        else:
            self._log(" No thread to join")

    def _run_sender(self):
        self._log("Message Client running")

        while self.is_running():
            stream_max_reached = False
            # Go through messages
            try:
                message_stream = []
                while self._message_queue.qsize() > 0 and len(message_stream) < self._stream_max:
                    message_stream.append(self._message_queue.get())
            
                if len(message_stream) > 0:
                    stream_max_reached = len(message_stream) >= self._stream_max
                    self._send(message_stream)
            
            except Exception as e:
                self._log("Message sending issue: " + str(e))

            # wait before checking again, unless stream max was reached and wait disabled
            if self._stream_max_wait or (not stream_max_reached and not self._stream_max_wait):
                time.sleep(self._wait_time_sec)

        self._log("Message Client exiting")

    def _send(self, message):
        if self._network_manager:
            self._network_manager.post(self.priority, self._send_function, message, is_stream=True)
        else:
            self._log("Sending message without network manager")
