import signal
from time import sleep

from paramiko.channel import ChannelFile

from logger import TestingLogger
from vms.dauth_directory_vm import DauthDirectoryVM
from vms.dauth_service_vm import DauthServiceVM
from vms.ueransim_vm import UeransimVM


class TestingConfig:
    """
    All configuration needed to connect to the VMs.
    """
    def __init__(self, vagrant_dir: str, config_dir: str) -> None:
        self.vagrant_dir = vagrant_dir
        self.config_dir = config_dir
        
        self.directory_host = "directory"
        self.service1_host = "colte1"
        self.service2_host = "colte2"
        self.service3_host = "colte3"
        self.service4_host = "colte4"
        self.ueransim_host = "ueransim"


class TestingState:
    """
    Holds all testing state, including vm connections.
    """

    def __init__(self, config: TestingConfig):
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
        sleep(3)

    def start_and_check_gnb(self, config_path: str) -> None:
        """
        Attempts to start a gnb and check that it was successful.
        Returns the gNB on success.
        Raises TestingException on failure.
        """
        gnb = self.ueransim.add_gnb(config_path)
        res = self._find_or_timeout("NG Setup procedure is successful", "[\x1b[31m\x1b[1merror\x1b[m]", gnb.stdout)
    
        if res is not None:
            TestingLogger.logger.error("Failed to start gNB: \n   {}".format(res))
            raise TestingException()
        
        return gnb
    
    def start_and_check_ue(self, config_path: str) -> None:
        """
        Attempts to start a gnb and check that it was successful.
        Returns the UE on success.
        Raises TestingException on failure.
        """
        ue = self.ueransim.add_ue(config_path)
        res = self._find_or_timeout("Connection setup for PDU session", "[\x1b[31m\x1b[1merror\x1b[m]", ue.stdout)
        
        if res is not None:
            TestingLogger.logger.error("Failed to start UE: \n   {}".format(res))
            raise TestingException()
        
        return ue
        
    def _find_or_timeout(self, success_string: str, error_string: str, 
                         stdout: ChannelFile, timeout_seconds: int=5) -> str:
        """
        Internal function that checks stdout for a particular string.
        Returns None on success, or stdout on failure.
        """
        res = []

        try:
            with Timeout(seconds=timeout_seconds, error_message="Failed to confirm successful"):
                for line in stdout:
                    if success_string in line:
                        return None
                    else: 
                        res.append(line.strip())
                        if error_string and error_string in line:
                            res.append("<<< Error string: '{}' >>>".format(error_string))
                            return "\n   ".join(res)
        except TimeoutError:
            res.append("<<< Timeout: {}s >>>".format(timeout_seconds))
            return "\n   ".join(res)

class TestingException(Exception):
    pass

class Timeout:
    """
    Handles timeouts in a 'with timeout(...)' block.
    Credit: https://stackoverflow.com/a/22348885 
    """
    def __init__(self, seconds=1, error_message='Timeout'):
        self.seconds = seconds
        self.error_message = error_message
    def handle_timeout(self, signum, frame):
        raise TimeoutError(self.error_message)
    def __enter__(self):
        signal.signal(signal.SIGALRM, self.handle_timeout)
        signal.alarm(self.seconds)
    def __exit__(self, type, value, traceback):
        signal.alarm(0)
