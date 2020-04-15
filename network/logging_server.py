import sys
import signal
import threading

from network.services import LoggingServer
from network.network_manager import NetworkManager

# This file is meant to serve as a way to run a network manager indefinitely

def run_server(port=13173):
    print('Starting Logging Server')
    nwm = NetworkManager(port=13173)
    nwm.add_service(LoggingServer())
    nwm.start()

    def stop_server(signal, frame):
        print('\nStopping Logging Server')
        nwm.stop()
        sys.exit(0)

    signal.signal(signal.SIGINT, stop_server)
    print("Ctr-c to stop")
    forever = threading.Event()
    forever.wait()


# Boot up and wait for a keyboard interrupt
if __name__ == "__main__":
    run_server()