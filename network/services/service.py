import os
import logging
import time

# The purpose of a service is to give a handle on messages
# This includes (possibly) holding onto stubs, priorities, etc.
# Services do not need to utilize start and stop, but they may be useful

# Notes:
# - 'name' and 'priority' can be overridden on instances without affecting class globals
#   -- Allows globals to act as defaults
#   -- Allows for duplicate services under different names/priorities

# Generic messaging service
class Service:
    # Used for identifying and managing services
    # Should be overwritten in extending class
    name = None
    priority = None

    # Used for logging across services
    _loggers = {}
    _logger_dir = "./output/service_logs"

    # Used as default until a network manager is added
    _network_manager = None

    # Called when adding service to network manager
    # should not need to be overwritten in most cases
    def set_network_manager(self, network_manager):
        self._network_manager = network_manager

    # Called to register a service to a gRPC server
    # THIS MUST BE OVERWRITTEN TO GET MESSAGES FROM THE SERVER
    # This function should utilize the appropriate gRPC add servicer function
    def add_service_to_server(self, server):
        pass

    # Called on server start
    def start(self):
        pass

    # Called on server stop
    def stop(self):
        pass

    # Called when the send function gets a reply
    # Can be used if the targeted server gives a meaningful reply
    def store_result(self, result, store_args=None):
        pass

    # Logs the message to the service
    def _log(self, message, tag=None):
        # name must be defined to log
        if not self.name:
            return

        # Create the logger if it does not exist
        if self.name not in self._loggers:
            os.makedirs(self._logger_dir, exist_ok=True)
            new_logger = logging.getLogger(self.name)
            new_logger.setLevel(logging.DEBUG)
            fh = logging.FileHandler(os.path.join(self._logger_dir, self.name + ".log"))
            fh.setLevel(logging.DEBUG)
            new_logger.addHandler(fh)
            self._loggers[self.name] = new_logger

        # Timestamp and log message
        timestamp = "[{0:.10f}]".format(time.time())
        if tag:
            timestamp += " ({0})".format(tag)
        self._loggers[self.name].debug(timestamp + " \"" + message + "\"")

