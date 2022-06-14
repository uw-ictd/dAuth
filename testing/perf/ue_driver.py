import atexit
import logging
import re
import socket
import struct
import subprocess
import threading
import time
import argparse

from enum import IntEnum

logging.basicConfig()
log = logging.getLogger(__name__)
log.setLevel(logging.INFO)

class UERANSIM_MESSAGE_KIND(IntEnum):
    EMPTY = 0
    ECHO = 1
    ERROR = 2
    RESULT = 3
    COMMAND = 4

class UeransimUe(object):
    def __init__(self, name):
        # Start the actual UE process
        # ToDo Make sure the imsi and keys make sense when generating many ues
        self.process_handle = subprocess.Popen(
            ["/home/vagrant/ueransim/nr-ue", 
             "-c", f"/home/vagrant/configs/ueransim/ue.yaml", 
             "--no-routing-config",
             "-i", name],
            stderr=subprocess.PIPE,
            stdout=subprocess.DEVNULL,
            #stdout=None,
            )
        # For now register a kill at exit for each process, I'm not sure if this breaks garbage collection though...
        atexit.register(lambda :self.process_handle.kill())

        # As a quick hack, the control port will be output immediately at startup on the
        # first line of stderr.
        self.ue_process_control_port = self._extract_control_port(self.process_handle.stderr.readline().decode("utf8"))

        # Setup the control socket
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind(("127.0.0.1", 0))
        self.sock.settimeout(5.0)
        log.info(f"Communicating with {name} at port {self.ue_process_control_port} from {self.sock.getsockname()}")

        # Communication metadata
        self.name = name
        self.ueransim_major = 3
        self.ueransim_minor = 2
        self.ueransim_patch = 6

    def _extract_control_port(self, port_string: str) -> int:
        """Extract an integer port from the port printed on startup by UERANSIM
        """
        for match in re.findall(r"^ControlPort\[([0-9]+)\]$", port_string):
            port: int = int(match)
        return port

    def _pack_message(self, data: str, kind: UERANSIM_MESSAGE_KIND) -> bytes:
        """Packs a message for transport with the ueransim wire protocol
        """
        encoded_node_name = self.name.encode("utf8")
        message = struct.pack(
            "!BBBbi", self.ueransim_major, self.ueransim_minor, self.ueransim_patch, int(kind), len(encoded_node_name)
        )
        message += encoded_node_name

        encoded_data = data.encode("utf8")
        message += struct.pack("!i", len(encoded_data))
        message += encoded_data

        return message

    def _unpack_message(self, data: bytes) -> str:
        log.debug("Received Msg: %s", str(data))
        header = struct.unpack("!BBBb", data[:4])
        name_length = struct.unpack("!i", data[4:8])[0]
        encoded_name = data[8:8+name_length]
        value_length = struct.unpack("!i", data[8 + name_length: 8 + name_length + 4])[0]
        encoded_value = data[8 + name_length + 4: 8 + name_length + 4 + value_length]
        if (8 + name_length + 4 + value_length) < len(data):
            log.warn("Extra bytes in message: %s", data[(8 + name_length + 4 + value_length):])
        return (encoded_name.decode("utf8"), encoded_value.decode("utf8"))

    def request_echo(self, data: str) -> str:
        message = self._pack_message(data, UERANSIM_MESSAGE_KIND.ECHO)
        self.sock.sendto(message, ("127.0.0.1", self.ue_process_control_port))
        try:
            return self._unpack_message(self.sock.recvfrom(1024)[0])
        except socket.timeout as e:
            print("timed out", e)

    def send_command(self, command: str) -> str:
        message = self._pack_message(command, UERANSIM_MESSAGE_KIND.COMMAND)
        self.sock.sendto(message, ("127.0.0.1", self.ue_process_control_port))
        try:
            return self._unpack_message(self.sock.recvfrom(1024)[0])
        except socket.timeout as e:
            raise ConnectionError("Command response not received")


def run_test_loop(ue: UeransimUe, interval: float, iterations: int):
    for i in range(iterations):
        print(ue.send_command("deregister sync-disable-5g"))
        # Sleeps here seem to help with open5gs stability : (
        time.sleep(0.5)
        print(ue.send_command("reconnect {}".format(i)))
        # Sleep for interval time, or at least long enough for stability
        time.sleep(max(0.5, interval))


def main() -> None:
    parser = argparse.ArgumentParser(
        description='Creates a number of UEs and prints connection performance stats'
    )

    parser.add_argument(
        "-n",
        "--num-ues",
        required=True,
        type=int,
        help="Number of UEs",
    )

    parser.add_argument(
        "-i",
        "--interval",
        required=True,
        type=int,
        help="Interval in milliseconds to connect and reconnect",
    )
    
    parser.add_argument(
        "-t",
        "--iterations",
        required=True,
        type=int,
        help="Number of times to reconnect",
    )
    
    args = parser.parse_args()
    
    num_ues = args.num_ues
    ues = []
    for i in range(num_ues):
        imsi = "imsi-90170{}".format(str(i+1).zfill(10))
        ues.append(UeransimUe(imsi))
    
    threads = []
    for ue in ues:
        threads.append(threading.Thread(
            target= lambda: run_test_loop(ue, args.interval*0.001, args.iterations)))
        threads[-1].start()

if __name__ == "__main__":
    main()
