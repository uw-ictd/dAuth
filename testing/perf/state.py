from time import sleep
from typing import List

from connections.directory_connection import DauthDirectoryConnection
from connections.service_connection import DauthServiceConnection
from connections.ueransim_connection import UeransimConnection


class ConnectionConfig:
    """
    Connection configuration options.
    """
    def __init__(self, hostname: str, id: str, username: str, port: int, keyfile: str) -> None:
        self.hostname: str = hostname
        self.id: str = id
        self.username: str = username
        self.port: int = port
        self.keyfile: str = keyfile
        
        # Only for vagrant vms
        self.service_ip: str = None
        self.directory_addr: str = None


class NetworkStateConfig:
    def __init__(self, config_dir: str, ue_driver_path: str) -> None:
        self.config_dir = config_dir
        self.ue_driver_path = ue_driver_path
        
        self.service_configs: List[ConnectionConfig] = list()
        self.directory_config: ConnectionConfig = None
        self.ueransim_config: ConnectionConfig = None


class VagrantNetworkStateConfig:
    """
    All state configuration needed to connect to the vagrant VMs.
    """
    def __init__(self, vagrant_dir: str, config_dir: str, ue_driver_path: str) -> None:
        self.vagrant_dir = vagrant_dir
        self.config_dir = config_dir
        self.ue_driver_path = ue_driver_path
        
        self.directory_host = "directory"
        self.service1_host = "colte1"
        self.service2_host = "colte2"
        self.service3_host = "colte3"
        self.service4_host = "colte4"
        self.ueransim_host = "ueransim"


class NetworkState:
    """
    Holds all network state, including ssh connections.
    """

    def __init__(self, config: NetworkStateConfig):
        self.config_dir: str = config.config_dir
        self.services: List[DauthServiceConnection] = list()

        self.directory = DauthDirectoryConnection(
            config.directory_config.hostname,
            config.directory_config.id,
            config.directory_config.username,
            config.directory_config.port,
            config.directory_config.keyfile,
        )
        self.directory.directory_addr = config.directory_config.directory_addr
        
        self.ueransim = UeransimConnection(
            config.ueransim_config.hostname,
            config.ueransim_config.id,
            config.ueransim_config.username,
            config.ueransim_config.port,
            config.ueransim_config.keyfile,
        )
        
        for service_config in config.service_configs:
            connection = DauthServiceConnection(
                service_config.hostname,
                service_config.id,
                service_config.username,
                service_config.port,
                service_config.keyfile,
            )
            connection.service_ip = service_config.service_ip

            self.services.append(connection)
        
        with open(config.ue_driver_path, "r") as f:
            self.ueransim.upload_file(f, "./ue_driver.py")

    def reset(self):
        """
        Resets all system state by restarting all services and
        clearing databases and keys.
        """
        self.ueransim.remove_devices()
        
        for service in self.services:
            service.stop_service()

        self.directory.stop_service()
        self.directory.remove_db()
        self.directory.start_service()
        
        for service in self.services:
            service.remove_db()
            service.remove_keys()
            service.start_service()
            
        # Not ideal, but UERANSIM seems to need this delay to work correctly.
        # Without it, immediately adding gNBs and UEs causes the first 
        # connection request to fail.
        sleep(2)
