import sys
sys.path.append("../protos")
sys.path.append("..")

import time
from services import DebugPing
from network_manager import NetworkManager

# This is simple test of the debugping service
# Contains two functions

class PingTester:
    def __init__(self, host='localhost', port=12115):
        # Used for main manager
        self._host = host
        self._port = port

    # Simple test to ping self
    def ping_self(self, num_pings=1, ping_delay=0):
        print("Running ping_self test")
        nwm, dp_service = self.make_manager("localhost", self._port)
        nwm.start()
        for _ in range(num_pings):
            dp_service.ping(self.address())
            time.sleep(ping_delay)
        time.sleep(1)
        results, remaining_messages = dp_service.get_results()
        self.process_results(results, self.address(), dp_service._creation_time, remaining_messages)
        nwm.stop()

    # Addresses is a list of address strings of form 'host:port'
    # Goes through each address one at a time
    # Pings each address num_pings times, with ping_delay time between each ping
    def ping_multiple(self, addresses, num_pings=1, ping_delay=0):
        print("Running ping test")

        # start local network manager
        nwm, dp_service = self.make_manager(self._host, self._port)
        nwm.start()

        # go through and test each address
        for address in addresses:
            for _ in range(num_pings):
                dp_service.ping(address)
                time.sleep(ping_delay)
            time.sleep(0.1)

        time.sleep(0.001*len(addresses)*num_pings)

        # process and print results, then stop network manager
        results, remaining_messages = dp_service.get_results()
        self.process_results(results, self.address(), dp_service._creation_time, remaining_messages)
        nwm.stop()
        
    # Makes a manager with the debug ping service
    # Returns the manager and the service
    def make_manager(self, host, port):
        dp_service = DebugPing()
        nwm = NetworkManager(host=host, port=port)
        nwm.add_service(dp_service)
        return nwm, dp_service

    def address(self, host=None, port=None):
        if not host:
            host = self._host
        if not port:
            port = self._port

        return host + ":" + str(port)


    # Processes and print results from ping tests
    def process_results(self, results, from_address, cap, remaining_messages):
        avg_set = {}
        max_set = {}
        min_set = {}
        for address, time_list in results.items():
            mn = cap
            mx = 0
            s = 0
            c = 0
            for t in time_list:
                if t < cap:
                    mn = min(mn, t)
                    mx = max(mx, t)
                    s += t
                    c += 1

            if c:
                avg_set[address] = s/c
                min_set[address] = mn
                max_set[address] = mx
            else:
                avg_set[address] = 0
                min_set[address] = 0
                max_set[address] = 0
        
        print("  Ping times for", from_address, "(" + str(remaining_messages) + " pings not yet received)")

        for address, avg in avg_set.items():
            print("   ", address, " -- ", "{0:1f}".format(avg*1000), " ms ", "(min ", "{0:1f}".format(min_set[address]*1000), ", max ", "{0:1f}".format(max_set[address]*1000), ")", sep="")


if __name__ == "__main__":
    tester = PingTester()
    tester.ping_multiple(["localhost:12115"])