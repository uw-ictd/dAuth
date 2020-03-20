import sys
sys.path.append("..")

import network_manager
import socket
import threading
import time


class PingTester:
    def __init__(self, host='127.0.0.1', port=12115, use_local=True, use_nw_manager=True):
        self._use_local = use_local
        self._host = host
        self._port = port
        self._server_thread = None
        self._server_running = False
        self._nw_manager = None

        if use_nw_manager:
            self._nw_manager = network_manager.NetworkManager()
            self._nw_manager.start()

    # Send a message with a time stamp
    def ping(self, priority=0):
        message = int(time.time()*1000000000) # time in ns
        ret = int(self._send(message, priority))
        return int(time.time()*1000000000) - int(ret)
    
    # Spawn thread to run server
    def start_server(self):
        if self._use_local and not self._server_thread:
            self._server_thread = threading.Thread(target=self._local_server)
            self._server_running = True
            self._server_thread.start()

    # Stop thread currently running server
    def stop_server(self):
        self._server_running = False
        if self._server_thread:
            self._server_thread.join()
            self._server_thread = None
    
    # Stop thread currently running server
    def stop_nw_manager(self):
        if self._nw_manager:
            self._nw_manager.stop()

    # Directly send message
    def _send(self, message, priority):
        r = None
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self._host, self._port))
            m = str(message).encode()

            # queue message
            if self._nw_manager:
                self._nw_manager.post(priority, s.sendall, m)
            # send directly
            else:
                s.sendall(m)

            r = s.recv(1024).decode()
        
        return r
        
    # For hosting a local server, returns the delay
    # TODO: just send back the timestamp?
    def _local_server(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            s.bind((self._host, self._port))
            s.settimeout(0.1)
            s.listen()

            while self._server_running:
                try:
                    conn, addr = s.accept()
                except socket.timeout:
                    continue

                with conn:
                    # data = conn.recv(1024)
                    # t = str(int(time.time() * 1000000000) - int(data.decode()))
                    # conn.sendall(t.encode())
                    conn.sendall(conn.recv(1024))

class PingTests:
    # Use to se
    use_local = True
    host = "127.0.0.1"
    port = 12117

    # Simply runs all available tests
    @staticmethod
    def run_all_tests():
        print(" --- Running all tests ---")
        print()

        print(" Simple Ping Test")
        for interval in [0.0001, 0.001, 0.01, 0.1]:
            print("   ", interval, "second interval:")
            for pings in [10, 100, 1000]:
                results_direct_send = PingTests.simple_ping(ping_interval=interval, num_pings=pings)
                avg = sum(results_direct_send) // len(results_direct_send)
                print("      Average ping time for direct send:", avg, "ns (" + str(avg / 1000000) + " ms) with", pings, "pings")
                results_nw_manager = PingTests.simple_ping(ping_interval=interval, num_pings=pings, use_nw_manager=True)
                avg = sum(results_nw_manager) // len(results_nw_manager)
                print("      Average ping time for nw manager: ", avg, "ns (" + str(avg / 1000000) + " ms) with", pings, "pings")
                print()

        print(" Single Priority Ping Comparison Test")
        for interval in [0.0001, 0.001, 0.01, 0.1]:
            print("   ", interval, "second interval:")
            for pings in [10, 100, 1000]:
                print("      Network manager is " + str(PingTests.compare_simple_pings(num_pings=pings, ping_interval=interval)) + "x slower with ", pings, "pings")

        print()
        print(" --- End of tests ---")


    # Tests that pings work, returns results of timing
    @staticmethod
    def simple_ping(num_pings=10, ping_interval=0.01, use_nw_manager=False):
        tester = PingTests._default_tester(use_nw_manager)
        time.sleep(1)
        results = []
        for _ in range(num_pings):
            results.append(tester.ping())
            time.sleep(ping_interval)

        tester.stop_nw_manager()
        tester.stop_server()
        return results

    # Does a simple ping test with and without the network manager
    # Returns the average of the two
    @staticmethod
    def compare_simple_pings(num_pings=10, ping_interval=0.01):
        results_direct_send = PingTests.simple_ping(num_pings=num_pings, ping_interval=ping_interval)
        avg_direct = sum(results_direct_send) // len(results_direct_send)
        results_nw_manager = PingTests.simple_ping(use_nw_manager=True)
        avg_nw_manager = sum(results_nw_manager) // len(results_nw_manager)
        return avg_nw_manager / avg_direct
        

    # Builds and starts a tester with PingTests defaults
    @staticmethod
    def _default_tester(use_nw_manager):
        tester = PingTester(host=PingTests.host, port=PingTests.port, use_local=PingTests.use_local, use_nw_manager=use_nw_manager)
        tester.start_server()
        return tester

if __name__ == "__main__":
    PingTests.run_all_tests()