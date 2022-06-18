import os

from logger import TestingLogger

from perf.config import ServiceConfig
from perf.setups.common import NetworkSetup
from perf.state import NetworkState


class HomeAuthSetup(NetworkSetup):
    def __init__(self, state: NetworkState) -> None:
        super().__init__(state)
        self.gnb_config_name: str  = "gnb-2.yaml"
    
    def _configure(self, num_users: int):
        TestingLogger.logger.info("Configuring for {} UE(s) in home auth".format(num_users))
    
        # Configure all unused state to use default empty config.
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

