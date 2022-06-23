import signal
from time import sleep

from paramiko.channel import ChannelFile

from logger import TestingLogger
from connections.directory_connection import DauthDirectoryConnection
from connections.service_connection import DauthServiceConnection
from connections.ueransim_connection import UeransimConnection


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
        
        self.error_log = "[\x1b[31m\x1b[1merror\x1b[m]"

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

    def start_and_check_gnb(self, config_path: str) -> None:
        """
        Attempts to start a gnb and check that it was successful.
        Returns the gNB on success.
        Raises TestingException on failure.
        """
        gnb = self.ueransim.add_gnb(config_path)
        res = self._find_or_timeout("NG Setup procedure is successful", 1, self.error_log, gnb.stdout)
    
        if res is not None:
            TestingLogger.logger.error("Failed to start gNB: \n   {}".format(res))
            raise TestingException()
        
        return gnb
    
    def start_and_check_ue(self, config_path: str, imsi: str, num_ues: int) -> None:
        """
        Attempts to start a gnb and check that it was successful.
        Returns the UE on success.
        Raises TestingException on failure.
        """
        ue = self.ueransim.add_ue(config_path, imsi, num_ues)
        res = self._find_or_timeout("Connection setup for PDU session", num_ues, None, ue.stdout)
        
        if res is not None:
            TestingLogger.logger.error("Failed to start UE: \n   {}".format(res))
            raise TestingException()
        
        return ue
        
    def _find_or_timeout(self, success_string: str, num_success: int, error_string: str, 
                         stdout: ChannelFile, timeout_seconds: int=5) -> str:
        """
        Internal function that checks stdout for a particular string.
        Returns None on success, or stdout on failure.
        """
        res = []
        success_found = 0

        try:
            with Timeout(seconds=timeout_seconds, error_message="Failed to confirm successful"):
                for line in stdout:
                    if success_string in line:
                        success_found += 1
                    else: 
                        res.append(line.strip())
                        if error_string and error_string in line:
                            res.append("<<< Error string: '{}', {} successes >>>".format(error_string,success_found))
                            return "\n   ".join(res)
                        
                    if success_found >= num_success:
                        return None
        except TimeoutError:
            res.append("<<< Timeout: {}s, {} successes >>>".format(timeout_seconds, success_found))
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