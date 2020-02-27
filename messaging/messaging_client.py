import sys, os
import queue
import logging
import time
import threading
import grpc
import messaging_pb2
import messaging_pb2_grpc
import services


# Simple messaging client which allows sending to multiple services
class MessagingClient:
    def __init__(self, host='localhost',
                       port=13127,
                       logfile_dir="./output/client_logs",
                       stream_max=10000,
                       stream_max_wait=False):
        
        # Logger
        self._logger = logging.getLogger("client")
        self._logger.setLevel(logging.DEBUG)
        os.makedirs(logfile_dir, exist_ok=True)
        fh = logging.FileHandler(os.path.join(logfile_dir, "messaging_client.log"))
        fh.setLevel(logging.DEBUG)
        self._logger.addHandler(fh)

        # message sending
        self._host = host
        self._port = port
        self._running = False
        self._sender_thread = None
        self._wait_time_sec = 1
        self._stream_max = stream_max  # Prevents one service from hogging
        self._stream_max_wait = stream_max_wait  # If stream_max is reached, wait before sending more

        # gRPC stub for sending messages
        self._stub = messaging_pb2_grpc.MessagingStub(grpc.insecure_channel(self._host + ':' + str(self._port)))

        # Map of services to sender functions
        self._service_functions = {
            services.LoggingService._service : self._stub.SendLogMessages,
        }

        # Message queues for each service
        self._service_message_queues = {}
        for service in self._service_functions.keys():
            self._service_message_queues[service] = queue.Queue()

    # Queues a message to be sent to the service
    def send_message(self, service, message):
        if service in self._service_message_queues:
            if self._running:
                self._service_message_queues[service].put(message)
            else:
                self._log(" Messaging Client not running, message ignored for " + service)
        else:
            self._log(" Invalid service: " + service)

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
            # Go through all service queues
            for service, message_queue in self._service_message_queues.items():
                try:
                    message_stream = []
                    while message_queue.qsize() > 0 and len(message_stream) < self._stream_max:
                        message_stream.append(message_queue.get())
                
                    if len(message_stream) > 0:
                        stream_max_reached = len(message_stream) >= self._stream_max
                        self._service_functions[service](iter(message_stream))
                
                except Exception as e:
                    self._log("Message sending issue (" + service + "): " + str(e))

            # wait before checking again, unless stream max was reached and wait disabled
            if self._stream_max_wait or (not stream_max_reached and not self._stream_max_wait):
                time.sleep(self._wait_time_sec)

        self._log("Message Client exiting")
    
    # Logging
    def _log(self, content):
        timestamp = "[{0:.10f}]".format(time.time())
        self._logger.debug(timestamp + " (MessagingClient) \"" + content + "\"")


# Global client for sending messages
class GlobalMessagingClient:
    _client = None

    @staticmethod
    def set_client(client):
        GlobalMessagingClient._client = client

    @staticmethod
    def start():
        if GlobalMessagingClient._client:
            GlobalMessagingClient._client.start()
        else:
            print("No Messaging Client to start")

    @staticmethod
    def stop():
        if GlobalMessagingClient._client:
            GlobalMessagingClient._client.stop()
        else:
            print("No Messaging Client to stop")

    # Send message to specified service through client
    @staticmethod
    def send_message(service, message):
        try:
            if GlobalMessagingClient._client and GlobalMessagingClient._client.is_running():
                GlobalMessagingClient._client.send_message(service, message)
            elif GlobalMessagingClient._client:
                print("Unable to send message to service", service, ", client not running")
            else:
                print("Unable to send message to service", service, ", no client to send")

        except Exception as e:
            print("Unable to send message to service", service, ", exception:", e)


# Wrapper for messaging the logging service
class GlobalLoggingClient:
    _host = None

    @staticmethod
    def set_host(host):
        # Make filename safe
        GlobalLoggingClient._host = "".join([c for c in host if c.isalpha() or c.isdigit() or c in ":-|_"])

    # Generate and send message to Logging Service
    @staticmethod
    def log(category, content):
        ts = "{0:.10f}".format(time.time())
        message = messaging_pb2.LogMessage(timestamp=ts, host=GlobalLoggingClient._host, category=category, content=content)
        GlobalMessagingClient.send_message(services.LoggingService._service, message)

    # Returns whether there are messages remaining to send
    # Mostly used for debug/performance testing
    @staticmethod
    def messages_left():
        return sum(q.qsize() for q in GlobalMessagingClient._client._service_message_queues.values())
