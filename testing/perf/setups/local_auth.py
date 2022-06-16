import os
from logging import INFO
from time import sleep

from logger import TestingLogger

from perf.config import ServiceConfig
from perf.setups.common import NetworkSetup
from perf.state import NetworkState
from perf.metrics import PerfMetrics


class LocalAuthSetup(NetworkSetup):
    def __init__(self, state: NetworkState) -> None:
        super().__init__(state)
        self.gnb_config_path: str  = "./configs/ueransim/gnb-1.yaml"
    
    def _configure(self, num_users: int):
        """
        Configures the network for the number of users and auth situation.
        """
        TestingLogger.logger.info("Configuring for {} UE(s) in local auth".format(num_users))
    
        # Configure all unused state to use default empty config.
        # For local authentication, only one service is used.
        self.state.service2.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service2.yaml")))
        self.state.service3.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service3.yaml")))
        self.state.service4.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service4.yaml")))

        service_config = ServiceConfig(
            os.path.join(self.state.config_dir, "service1.yaml"))

        sqn_slice_max = {0: 32}
        backup_network_ids = dict()

        if num_users < 1:
            raise Exception("Number of users is less than 1")
        elif num_users > 9999999999:  # can represent 901700000000001-901709999999999
            raise Exception("Too many users to represent")
        else:
            for i in range(num_users):
                imsi = "imsi-90170{}".format(str(i+1).zfill(10))
                service_config.add_user(imsi, sqn_slice_max, backup_network_ids)
        
        self.state.service1.change_config(service_config)

    def run_perf(self, num_ues: int, interval: int, iterations: int):
        """
        Configures the network for the provided setup.
        Runs and prints the resulting performance metrics.
        """
        TestingLogger.logger.info("Running local auth perf")
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

