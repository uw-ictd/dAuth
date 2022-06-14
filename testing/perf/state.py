import signal
from time import sleep

from paramiko.channel import ChannelFile

from logger import TestingLogger
from vms.dauth_directory_vm import DauthDirectoryVM
from vms.dauth_service_vm import DauthServiceVM
from vms.ueransim_vm import UeransimVM


class NetworkStateConfig:
    """
    All state configuration needed to connect to the VMs.
    TODO: Generalize for any ssh connections.
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
        self.vagrant_dir = config.vagrant_dir
        self.config_dir = config.config_dir
        
        self.directory: DauthDirectoryVM = \
            DauthDirectoryVM(config.vagrant_dir, config.directory_host)
        self.service1: DauthServiceVM = \
            DauthServiceVM(config.vagrant_dir, config.service1_host)
        self.service2: DauthServiceVM = \
            DauthServiceVM(config.vagrant_dir, config.service2_host)
        self.service3: DauthServiceVM = \
            DauthServiceVM(config.vagrant_dir, config.service3_host)
        self.service4: DauthServiceVM = \
            DauthServiceVM(config.vagrant_dir, config.service4_host)
        self.ueransim: UeransimVM = \
            UeransimVM(config.vagrant_dir, config.ueransim_host)
        self.services = (self.service1, self.service2, self.service3, self.service4)
        
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
        sleep(1)
