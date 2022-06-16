from typing import Union
from paramiko.channel import ChannelFile, ChannelStderrFile
from time import sleep

from perf.state import NetworkState
from perf.metrics import PerfMetrics
from logger import TestingLogger


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
        TestingLogger.logger.info("Running perf test")
        TestingLogger.logger.info(
            "Num UEs: {}, Inteval: {}ms, iterations: {}"
            .format(num_ues, interval, iterations))
        
        try:
            # configure and reset the network state 
            self._configure(num_ues)
            self.state.reset()
            
            # wait for network to settle
            TestingLogger.logger.info("Waiting for network to settle")
            sleep(5)
            
            # Start gnb and ues
            TestingLogger.logger.info("Starting gNB and UEs")
            self._start_gnb()
            output, err = self._start_ues(num_ues, interval, iterations)
            
            TestingLogger.logger.info("Processing output (varies by iterations*interval)")
            metrics = PerfMetrics()
            for line in output:
                try:
                    metrics.add_result_from_json(line)
                except Exception as e:
                    TestingLogger.logger.debug("Failed<{}>: {}".format(e, line.rstrip()))
                    
                    if "[error]" in line:
                        TestingLogger.logger.error("UERANSIM error detected: {}".format(line.rstrip()))
                
            for line in err:
                TestingLogger.logger.debug("Stderr:", line.rstrip())
                
            print("Results:")
            for name in metrics.get_names():
                print(" ", name)
                print("   cmd tags:", metrics.get_command_tags(name))
                print("   values  :", metrics.get_results(name))
                print("   averages:", metrics.get_average(name))
            print(" ", "All")
            print("   averages:", metrics.get_total_average())

        except Exception as e:
            TestingLogger.logger.error("Failed to run: {}".format(e))
        
        TestingLogger.logger.info("Perf completed")
