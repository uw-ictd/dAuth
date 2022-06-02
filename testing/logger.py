import logging
from typing import Union

from paramiko.channel import ChannelFile, ChannelStderrFile, ChannelStdinFile


logging.basicConfig()
logger = logging.getLogger("dauth-testing")
logger.setLevel(logging.DEBUG)


class TestingLogger:
    """
    Logging class that handles specific logging cases for testing.
    """
    logger = logger
    
    @staticmethod
    def log_cammand(host_name: str, command: str) -> None:
        """
        Logs the command being run.
        """
        logger.debug("{}: '{}'".format(host_name, command))

    @staticmethod
    def log_command_streams(command: str, 
        streams: Union[ChannelStdinFile, ChannelFile, ChannelStderrFile]) -> None:
        """
        Logs stdin and stderr from the provided command.
        """
        logger.debug("{} -> sout: '{}', serr: '{}'".format(command, 
            streams[1].read().decode(), streams[2].read().decode()))

    @staticmethod
    def log_command_execution(command: str, stdout: str, stderr: str) -> None:
        """
        Logs stdin and stderr from the provided command.
        """
        logger.debug("{} -> sout: '{}', serr: '{}'".format(command, stdout, stderr))
