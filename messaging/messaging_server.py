import sys, os
import logging
import time
import queue
import threading
import signal
import grpc
import messaging_pb2
import messaging_pb2_grpc
import services
from concurrent import futures


# Handles new messages
class Servicer(messaging_pb2_grpc.MessagingServicer):
    def __init__(self, services, logger):
        self._services = services
        self._logger = logger

    # Called on recieve of log message
    def SendLogMessages(self, request_iterator, context):
        for request in request_iterator:
            self._services[services.LoggingService._service].message_in(request)
        return messaging_pb2.LogResponse()


class MessagingServer():
    def __init__(self, host="localhost", 
                       port=13127,
                       logfile_dir="./output/server_logs"):

        # Services
        self._services = {
            services.LoggingService._service : services.LoggingService(),
        }

        # Logger
        self._logger = logging.getLogger("server")
        self._logger.setLevel(logging.DEBUG)
        os.makedirs(logfile_dir, exist_ok=True)
        fh = logging.FileHandler(os.path.join(logfile_dir, "messaging_server.log"))
        fh.setLevel(logging.DEBUG)
        self._logger.addHandler(fh)

        # gRPC server
        self._host = host
        self._port = port
        self._server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
        messaging_pb2_grpc.add_MessagingServicer_to_server(Servicer(self._services, self._logger), self._server)
        self._server.add_insecure_port(self._host + ":" + str(self._port))

    # Starts the receiver and processor, spawns threads
    def start(self):
        self._log("Start called on Messaging Server:")
        # Stop all services
        for service_name, service in self._services.items():
            try:
                self._log(" Starting " + service_name)
                service.start()
            except Exception as e:
                self._log("  --Failed to start " + service_name + ": " + str(e))
        
        # Stop gRPC server
        self._log(" Starting gRPC Server")
        self._server.start()
        
    # Calls stop method on all services
    # Stops gRPC server
    def stop(self, join=False):
        self._log("Stop called on Messaging Server:")
        # Stop all services
        for service_name, service in self._services.items():
            try:
                self._log(" Stopping " + service_name)
                service.stop()
            except Exception as e:
                self._log("  --Failed to stop " + service_name + ": " + str(e))
        
        # Stop gRPC server
        self._log(" Stopping gRPC Server")
        self._server.stop(None)

    # Logging
    def _log(self, content):
        timestamp = "[{0:.10f}]".format(time.time())
        self._logger.debug(timestamp + " (MessagingServer) \"" + content + "\"")


# Boot up and wait for a keyboard interrupt
if __name__ == "__main__":
    print('Starting Messaging Server')
    ms = MessagingServer()
    ms.start()

    def stop_server(signal, frame):
        print('\nStopping Messaging Server')
        ms.stop()
        sys.exit(0)

    signal.signal(signal.SIGINT, stop_server)
    print("Ctr-c to stop")
    forever = threading.Event()
    forever.wait()