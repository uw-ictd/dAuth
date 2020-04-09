import sys
sys.path.append("../protos")
sys.path.append("..")
import signal
import threading

import services
from network_manager import NetworkManager

# This file is meant to serve as a way to run a network manager indefinitely

# Boot up and wait for a keyboard interrupt
if __name__ == "__main__":
    print('Starting Network Manager')
    nwm = NetworkManager(host="172.18.0.1", port=13173)
    nwm.add_service(services.DebugPing())
    nwm.start()

    def stop_server(signal, frame):
        print('\nStopping Network Manager')
        nwm.stop()
        sys.exit(0)

    signal.signal(signal.SIGINT, stop_server)
    print("Ctr-c to stop")
    forever = threading.Event()
    forever.wait()