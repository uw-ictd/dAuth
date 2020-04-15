import threading
import logging
import os
import grpc
import time
from concurrent.futures import ThreadPoolExecutor
from network.network_queue import NetworkPriotyQueue, NetworkMessage


# TODO:
# - Add tc options to limit bandwidth


# Manages all network communication via a network priority queue
# Maintains services, which are the easiest way to gain access to the gRPC server
class NetworkManager:
    # Params:
    #  Operation:
    # - host: the ip/host of the gRPC server (not really used outside of logging info)
    # - port: the port to run the gRPC server on
    # - known_priorities: a list of priorities that will be used
    # - limit_to_known_priorities: does not allow new priorities after initializing known priorities
    #  
    #  Performance (defaults work well):
    # - block_size: Number of messages to pull from the queue at a time, or <1 to pull all available messages
    #               higher numbers can be more efficient, but may not work well if priorities must be followed closely
    # - sleep_time: delay before checking for new messages (higher results in less cpu usage, but higher potential latency)
    # - use_thread_pool: uses a thread pool instead of spawing threads for each message. Less overhead and more efficient in most cases
    # - thread_pool_workers: Number of workers for the thread pool, ignored if no thread pool is used
    # 
    #  Other:
    # - logfile_dir: all logfiles will be stored there
    def __init__(self, host="localhost", port="13172", known_priorities=None, limit_to_known_priorities=False,
                 block_size=1, sleep_time=0.0001, use_thread_pool=True, thread_pool_workers=10,
                 logfile_dir="./output/network_manager"):
        # internal
        self._service_manager = None
        self._queue = NetworkPriotyQueue(known_priorities=known_priorities, use_only_known=limit_to_known_priorities)
        self._sender_thread = None
        self._running = False
        self._thread_pool = None

        # logging
        self._logger = logging.getLogger("network_manager")
        self._logger.setLevel(logging.DEBUG)
        os.makedirs(logfile_dir, exist_ok=True)
        fh = logging.FileHandler(os.path.join(logfile_dir, "network_manager.log"))
        fh.setLevel(logging.DEBUG)
        self._logger.addHandler(fh)

        # operation
        self._host = host
        self._port = port
        self._address = self._host + ":" + str(self._port)
        self._server = grpc.server(ThreadPoolExecutor(max_workers=10))
        self._server.add_insecure_port("0.0.0.0:"+str(self._port))

        # performance
        self._sleep_time = sleep_time
        self._thread_pool_workers = thread_pool_workers
        self._use_thread_pool = use_thread_pool
        self._block_size = block_size

        # Add an empty service manager
        self._service_manager = ServiceManager(self)


    # Creates and queues a message to be sent
    def post(self, priority, stub_send_function, message,
             is_stream=False, store_func=None, store_args=None):
        self._queue.put(NetworkMessage(priority, stub_send_function, message,
                                      is_stream=is_stream,
                                      store_func=store_func, store_args=store_args))

    def has_messages(self):
        return not self._queue.empty()

    def is_running(self):
        return self._sender_thread and self._running

    # Returns the address of the gRPC server
    def address(self):
        return self._address

    # Starts the network manager, gRPC server, and all services
    def start(self):
        self._log("network manager start called")
        if self._sender_thread and self._running:
            self._log(" sender already active")
            return

        if self._sender_thread or self._running:
            self._log(" sender_thread and running mismatch")

        if self._use_thread_pool:
            self._thread_pool = ThreadPoolExecutor(max_workers=self._thread_pool_workers)

        if self._service_manager:
            self._service_manager.start_services()
            
        self._sender_thread = threading.Thread(target=self._sender)
        self._running = True
        self._sender_thread.start()

        self._log("Starting gRPC server on " + self.address())
        self._server.start()

    # Stops the network manager, gRPC server, and all services
    def stop(self):
        self._log("network manager stop called")
        if not self._sender_thread and not self._running:
            self._log(" sender not running")
            return

        self._running = False
        if self._sender_thread:
            self._sender_thread.join()
            self._sender_thread = None

        if self._use_thread_pool:
            self._thread_pool.shutdown()
            self._thread_pool = None

        if self._service_manager:
            self._service_manager.stop_services()

        self._server.stop(None)


    # Gets a service by name if it exists, None otherwise
    def get_service(self, service_name):
        self._log("Getting service: \"" + service_name + "\"")
        return self._service_manager.get_service(service_name)

    # Adds a service to the current service manager
    def add_service(self, service):
        self._log("Adding new service: \"" + service.name + "\"")
        service.add_service_to_server(self._server)
        self._service_manager.add_service(service)


    # Get next message(s), return as list
    def _get_next(self):
        if self._block_size == 1:
            message = self._queue.get()
            return [message] if message else None
        else:
            return self._queue.get_multiple(self._block_size if self._block_size > 1 else self._queue.size())

    # TODO: Chunking 
    # Internal function meant to be run by the sender thread
    def _sender(self):
        self._log("sender entering")
        while self._running:
            # get a list of avaiable messages
            messages = self._get_next()

            # If messages are found, send them
            if messages:
                if self._thread_pool:
                    self._thread_pool.map(self._send, messages)
                else:
                    for message in messages:
                        threading.Thread(target=self._send, args=(message,)).start()

            # Otherwise, sleep before checking again
            else:
                time.sleep(self._sleep_time)
    
        self._log("sender exiting")

    # Intended target for sending threads
    def _send(self, message):
        # self._log("sending message of size: " + str(message.size()))
        message.send()

    def _log(self, message, tag=None):
        # timestamp and log message
        timestamp = "[{0:.10f}]".format(time.time())
        if tag:
            timestamp += " ({0})".format(tag)
        self._logger.debug(timestamp + " \"" + str(message) + "\"")


# The service manager is used to provide easier control over a collection of services
# It is designed to be used by the network manager, not directly
class ServiceManager:
    def __init__(self, network_manager):
        self._network_manager = network_manager
        self._services = {}

    # Start all services
    def start_services(self):
        for service in self._services.values():
            try:
                self._log(" Starting " + service.name)
                service.start()
            except Exception as e:
                self._log("  --Failed to start " + service.name + ": " + str(e))

    # Stop all services
    def stop_services(self):
        for service_name, service in self._services.items():
            try:
                self._log(" Stopping " + service_name)
                service.stop()
            except Exception as e:
                self._log("  --Failed to stop " + service_name + ": " + str(e))

    # Returns a mapping of service names to services
    def service_map(self):
        return self._services

    # Returns a list of all services
    def services(self):
        return self._services.values()

    # Return a particular service by name
    def get_service(self, service_name):
        return self._services.get(service_name)

    # Adds the service to the network manager
    # Attaches to gRPC server to receive messages
    def add_service(self, service):
        self._services[service.name] = service
        service.set_network_manager(self._network_manager)
        if self._network_manager.is_running():
            self._start_service(service)

    # Attempts to run the start method on the service
    def _start_service(self, service):
        try:
            self._log(" Starting " + service.name)
            service.start()
        except Exception as e:
            self._log("  --Failed to start " + service.name + ": " + str(e))

    # Logs message to internal logger
    def _log(self, message):
        self._network_manager._log(message, tag="Service Manager")
