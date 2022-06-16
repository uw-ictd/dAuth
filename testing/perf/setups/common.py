from typing import Union
from perf.state import NetworkState
from paramiko.channel import ChannelFile, ChannelStderrFile


class NetworkSetup:
    """
    General network setup for performance testing.
    """
    
    def __init__(self, state: NetworkState) -> None:
        self.state: NetworkState = state
        self.gnb_config_path: str = None
        
    def _configure(self, num_users: int):
        """
        Configures the network for the number of users and auth situation.
        """
        pass

    def _start_gnb(self) -> None:
        """
        Starts the gnb for this setup.
        """
        if self.gnb_config_path:
            self.state.ueransim.add_gnb(self.gnb_config_path)
        else:
            raise Exception("GNB config path not set")
        
    def _start_ues(self, num_ues: int, interval: int, iterations: int) -> Union[ChannelFile, ChannelStderrFile]:
        """
        Runs the ue driver with the provided arguments and returns the output stream.
        """
        command = " ".join(
            ["sudo", "python3", "./ue_driver.py",
             "-n", str(num_ues),
             "-i", str(interval),
             "-t", str(iterations),
             ])

        res = self.state.ueransim.run_input_command(command)
        return res[1], res[2]

    def run_perf(self, num_ues: int, interval: int, iterations: int):
        """
        Configures the network for the provided setup.
        Runs and prints the resulting performance metrics.
        """
        pass