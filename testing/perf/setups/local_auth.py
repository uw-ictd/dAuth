import os

from logger import TestingLogger

from perf.config import ServiceConfig
from perf.setups.common import NetworkSetup
from perf.state import NetworkState
from perf.exception import PerfException


class LocalAuthSetup(NetworkSetup):
    def __init__(self, state: NetworkState) -> None:
        super().__init__(state)
        self.gnb_index = 0

    def setup_name(self) -> str:
        return "local_auth:<H>({})".format(
            self.state.services[0].id)
    
    def get_dauth_stats(self) -> str:
        return self.state.services[0].get_metrics()
    
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

        sqn_slice_max = {0: 32}
        backup_network_ids = dict()

        if num_users < 1:
            raise PerfException("Number of users is less than 1")
        elif num_users > 9999999999:  # can represent 901700000000001-901709999999999
            raise PerfException("Too many users to represent")
        else:
            for i in range(num_users):
                imsi = "imsi-91054{}".format(str(i+1).zfill(10))
                service_config.add_user(imsi, sqn_slice_max, backup_network_ids)
        
        main_service.change_config(service_config)
