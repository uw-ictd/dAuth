import time
import grpc
from network.protos import debugping_pb2, debugping_pb2_grpc
from network.services import Service

# Simple debug ping, records round-trip ping time for all hosts
# Demonstrates how services work with the network/service managers and gRPC
class DebugPing(Service, debugping_pb2_grpc.DebugPingServicer):
    name = "DebugPing"
    priority = 2

    def __init__(self):
        self._stubs = {}

        # Ping measurment results
        self._results = {}
        self._creation_time = time.time()
        self._active_pings = 0


    # REQUIRED FUNCTIONS

    # Add self to server
    def add_service_to_server(self, server):
        debugping_pb2_grpc.add_DebugPingServicer_to_server(self, server)


    # gRPC SERVICER FUNCTIONS

    # Called when receiving a new ping
    def DebugPing(self, request, context):
        return debugping_pb2.PingReply(time=request.time)


    # SERVICE SPECIFIC FUNCTIONS

    # Sends a ping to the provided address
    # Address is of the form 'host:port'
    # NOT THREAD SAFE
    def ping(self, address):
        self._active_pings += 1
        stub = self.get_stub(address)
        self._send(stub.DebugPing, debugping_pb2.PingMessage(time=time.time()), address)

    # Sends a ping to all known hosts
    def ping_all(self):
        for address in self._stubs.keys():
            self.ping(address)

    # Get a stub from an address, make a new one if one is not found
    def get_stub(self, address, make_new=True):
        if address not in self._stubs and make_new:
            self._make_stub(address)

        return self._stubs.get(address)

    # Get the address that the network manager's gRPC server runs on
    def address(self):
        if self._network_manager:
            return self._network_manager.address()
        else:
            return None

    # Create new stubs for a list of host addresses
    def add_hosts(self, host_addresses):
        for address in host_addresses:
            self._make_stub(address)

    # Returns results and active pings
    def get_results(self):
        return self._results, self._active_pings

    # Store args is the address of the server that was pinged
    def store_result(self, result, store_args=None):
        self._active_pings -= 1
        self._results[store_args].append(time.time() - result.time)

    # Create a new stub for the provided address
    # Also create a result entry
    def _make_stub(self, address):
        self._stubs[address] = debugping_pb2_grpc.DebugPingStub(grpc.insecure_channel(address))
        self._results[address] = []

    # Internal send function, passes message to service manager
    def _send(self, stub_send_function, ping_message, send_address):
        if self._network_manager:
            self._network_manager.post(self.priority, stub_send_function, ping_message,
                                       store_func=self.store_result, store_args=send_address)

    def _log(self, message, tag=None):
        if tag:
            super()._log(message, tag)
        else:
            super()._log(message, self.address())
