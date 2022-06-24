from typing import List, Union
from paramiko.channel import ChannelFile, ChannelStderrFile
from time import sleep
from os import path

from perf.state import NetworkState
from perf.metrics import PerfMetrics
from perf.config import UEConfig, GNBConfig
from perf.exception import PerfException
from logger import TestingLogger


class NetworkSetup:
    """
    General network setup for performance testing.
    """
    
    def __init__(self, state: NetworkState) -> None:
        self.state: NetworkState = state
        
        self._max_ues_per_gnb = 10
        self._max_gnbs = 10
        self._temp_dir = "/tmp/ueransim-perf-configs"
        
        self.gnb_index = 0

    def setup_name(self) -> str:
        pass
        
    def _configure(self, num_users: int) -> None:
        """
        Configures the network for the number of users and auth situation.
        """
        pass
    
    def _after_settle(self) -> None:
        """
        Operations to run after the network has time to settle.
        """
        pass
    
    def _build_configs(self, num_ues: int, amf_addr: str) -> List[str]:
        """
        Generates all gNB and UE configs at the UERANSIM temp config location.
        """
        
        num_gnbs = num_ues // self._max_ues_per_gnb
        
        if num_ues % self._max_gnbs > 0:
            num_gnbs += 1
            
        if num_gnbs > self._max_gnbs:
            raise PerfException("Max number of UEs exceeded")
        
        # clear out previous configs if they exist
        self.state.ueransim.run_command(" ".join(["rm", "-rf", self._temp_dir]))
        self.state.ueransim.run_command(" ".join(["mkdir", self._temp_dir]))
        
        gnb_paths = list()

        # create and upload configs for all ues/gnbs
        for i in range(num_gnbs):
            gnb_config = GNBConfig(path.join(self.state.config_dir, "gnb.yaml"))
            ue_config = UEConfig(path.join(self.state.config_dir, "ue.yaml"))
            
            ip = "127.0.0.{}".format(200 + i)
            
            if i < 56:
                gnb_config.set_ip(ip)
                gnb_config.set_nci(i+1)
                gnb_config.set_amf_addr(amf_addr)
            else:
                raise PerfException("Cannot represent IP")
            
            ue_config.set_gnb_search_list([ip])
            
            self.state.ueransim.run_command("sudo ip addr add {} dev enp0s8".format(ip))
            
            gnb_config_path = path.join(self._temp_dir, "gnb{}.yaml".format(i))
            gnb_paths.append(gnb_config_path)
            
            self.state.ueransim.upload_file(
                gnb_config.get_file(), 
                gnb_config_path
            )
            
            self.state.ueransim.upload_file(
                ue_config.get_file(), 
                path.join(self._temp_dir, "ue{}.yaml".format(i))
            )
            
        return gnb_paths
    
    def _start_gnbs(self, gnb_paths: List[str]) -> None:
        """
        Starts all of the gnbs for this setup.
        """
        if gnb_paths:
            TestingLogger.logger.info("Distributing UEs across {} gNB(s)".format(len(gnb_paths)))
            
            for config_path in gnb_paths:
                self.state.ueransim.add_gnb(config_path)
        else:
            raise PerfException("GNB configs not specified")
        
    def _start_ues(self, num_ues: int, interval: int, iterations: int) -> Union[ChannelFile, ChannelStderrFile]:
        """
        Runs the ue driver with the provided arguments and returns the output stream.
        """
        command = " ".join(
            ["sudo", "python3", "./ue_driver.py",
             "-n", str(num_ues),
             "-i", str(interval),
             "-t", str(iterations),
             "-c", self._temp_dir,
             ])

        res = self.state.ueransim.run_input_command(command)
        return res[1], res[2]

    def get_dauth_stats(self) -> str:
        pass

    def run_perf(self, num_ues: int, interval: int, iterations: int):
        """
        Configures the network for the provided setup.
        Runs and prints the resulting performance metrics.
        """
        TestingLogger.logger.info("Running perf test")
        
        if num_ues > self._max_gnbs * self._max_ues_per_gnb:
            raise PerfException("Too many UEs for max number of gNBS")
        
        TestingLogger.logger.info(
            "Num UEs: {}, Inteval: {}ms, iterations: {}"
            .format(num_ues, interval, iterations))
        
        try:
            # configure and reset the network state 
            self._configure(num_ues)
            self.state.reset()
            
            # wait for network to settle
            TestingLogger.logger.info("Waiting for network to settle")
            sleep(10)
            
            TestingLogger.logger.info("Running after-settle commands")
            self._after_settle()
            
            TestingLogger.logger.info("Building configs")
            gnb_paths = self._build_configs(num_ues, self.state.services[self.gnb_index].get_amf_ip())
            
            # tart gnb and ues
            TestingLogger.logger.info("Starting gNB and UEs")
            self._start_gnbs(gnb_paths)
            output, err = self._start_ues(num_ues, interval, iterations)
            
            TestingLogger.logger.info("Processing output (varies by iterations*interval)")
            metrics = PerfMetrics(self.setup_name() + ":<n,i,t>({},{},{})".format(num_ues, interval, iterations))
            for line in output:
                try:
                    metrics.add_result_from_json(line)
                except Exception as e:
                    TestingLogger.logger.debug("Failed<{}>: {}".format(e, line.rstrip()))
                    
                    if "[error]" in line:
                        TestingLogger.logger.error("UERANSIM error detected: {}".format(line.rstrip()))
                
            for line in err:
                TestingLogger.logger.debug("Stderr: " + line.rstrip())
                
            TestingLogger.logger.info("Results:")
            for name in metrics.get_names():
                TestingLogger.logger.info("  " + name)
                TestingLogger.logger.info("   cmd tags: " + str(metrics.get_command_tags(name)))
                TestingLogger.logger.info("   values  : " + str(metrics.get_results(name)))
                TestingLogger.logger.info("   averages: " + str(metrics.get_average(name)))
            TestingLogger.logger.info("  All")
            TestingLogger.logger.info("   averages: " + str(metrics.get_total_average()))
            
            TestingLogger.logger.info("Waiting for metrics")
            sleep(5)
            
            print(metrics.get_results_json())
            print(self.get_dauth_stats())

        except PerfException as e:
            TestingLogger.logger.error("Failed to run: {}".format(e))
        
        TestingLogger.logger.info("Perf completed")
        self.state.ueransim.remove_devices()
