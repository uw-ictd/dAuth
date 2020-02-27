import os
import logging
import time
import queue
import threading

# Generic messaging service
class MessagingService:
    _service = None
    _loggers = {}
    _logger_dir = "./output/service_logs"

    # Called on a new message to the service
    def message_in(self, message):
        pass

    # Called on server start
    def start(self):
        pass

    # Called on server stop
    def stop(self):
        pass

    # Logs the message to the service
    def _log(self, message):
        # Service must be defined to log
        if not self._service:
            return

        # Create the logger if it does not exist
        if self._service not in self._loggers:
            os.makedirs(self._logger_dir, exist_ok=True)
            new_logger = logging.getLogger(self._service)
            new_logger.setLevel(logging.DEBUG)
            fh = logging.FileHandler(os.path.join(self._logger_dir, self._service + ".log"))
            fh.setLevel(logging.DEBUG)
            new_logger.addHandler(fh)
            self._loggers[self._service] = new_logger

        # Timestamp and log message
        timestamp = "[{0:.10f}]".format(time.time())
        self._loggers[self._service].debug(timestamp + " ({0})".format(self._service) + " \"" + message + "\"")


# Server-side ervice for processing logs from multiple sources
class LoggingService(MessagingService):
    # Name used to identify service
    _service = "Logging"
    
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


    ####################
    # Service methods
    ####################

    def message_in(self, message):
        self._message_queue.put(message)


    ####################
    # Control methods
    ####################

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


    #####################
    # Internal methods
    #####################

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

