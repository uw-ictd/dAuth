import os

from logger import TestingLogger

from perf.config import ServiceConfig
from perf.setups.common import NetworkSetup
from perf.state import NetworkState
<<<<<<< HEAD
from perf.exception import PerfException
=======
>>>>>>> main


class LocalAuthSetup(NetworkSetup):
    def __init__(self, state: NetworkState) -> None:
        super().__init__(state)
<<<<<<< HEAD
        self.gnb_index = 0
    
    def _configure(self, num_users: int):
        TestingLogger.logger.info("Configuring for {} UE(s) in local auth".format(num_users))
        
        if len(self.state.services) < 1:
            raise PerfException("At least 1 service needed for local auth")
    
        # Configure all state to defaults
        for service in self.state.services[1:]:
            service_config = ServiceConfig(os.path.join(self.state.config_dir, "service.yaml"))
            service_config.set_directory_addr(self.state.directory.get_directory_addr())
            service_config.set_host_addr(service.get_host_addr())
            service_config.set_id(service.id)
            service.change_config(service_config)

        main_service = self.state.services[0]
        service_config = ServiceConfig(os.path.join(self.state.config_dir, "service.yaml"))
        service_config.set_directory_addr(self.state.directory.get_directory_addr())
        service_config.set_host_addr(main_service.get_host_addr())
        service_config.set_id(main_service.id)
=======
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
>>>>>>> main

        sqn_slice_max = {0: 32}
        backup_network_ids = dict()

        if num_users < 1:
<<<<<<< HEAD
            raise PerfException("Number of users is less than 1")
        elif num_users > 9999999999:  # can represent 901700000000001-901709999999999
            raise PerfException("Too many users to represent")
=======
            raise Exception("Number of users is less than 1")
        elif num_users > 9999999999:  # can represent 901700000000001-901709999999999
            raise Exception("Too many users to represent")
>>>>>>> main
        else:
            for i in range(num_users):
                imsi = "imsi-90170{}".format(str(i+1).zfill(10))
                service_config.add_user(imsi, sqn_slice_max, backup_network_ids)
        
<<<<<<< HEAD
        main_service.change_config(service_config)
=======
        self.state.service1.change_config(service_config)
>>>>>>> main

