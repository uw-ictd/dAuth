import subprocess
import io
from typing import IO, Union

from paramiko import SFTPClient, SSHConfig, SSHClient, AutoAddPolicy
from paramiko.channel import ChannelFile, ChannelStderrFile, ChannelStdinFile

from logger import TestingLogger


class Connection:
    """
    A class representing a single connection of any kind. Meant to be subclassed.
    Uses paramiko to maintain ssh connection, connects on build.
    """

    def __init__(self, hostname: str, id: str, username: str,  port: int, keyfile: str) -> None:
        self.hostname: str = hostname
        self.id: str = id
        self.username: str = username
        self.port: int = int(port)
        self.keyfile: str = keyfile
        
        self.ssh_client: SSHClient = self.build_ssh_client()
        self.sfpt_client: SFTPClient = self.ssh_client.open_sftp()

    def build_ssh_client(self) -> SSHClient:
        """
        Builds and returns an ssh client to the node.
        """
        
        ssh_client = SSHClient()
        ssh_client.set_missing_host_key_policy(AutoAddPolicy())
        ssh_client.connect(
            self.hostname,
            port=self.port,
            username=self.username,
            key_filename=self.keyfile
        )
    
        return ssh_client

    def run_command(self, command: str) -> Union[str, str]:
        """
        Runs the provided command in in the home dir of the connected machine.
        Returns a tuple of the resulting stdout and stderr.
        """
        outputs = self.run_input_command(command)
        stdout, stderr = outputs[1].read().decode(), outputs[2].read().decode()
        return (stdout, stderr)

    def run_input_command(self, command: str) -> Union[ChannelStdinFile, ChannelFile, ChannelStderrFile]:
        """
        Runs the provided command in in the home dir of the connected machine.
        Returns active streams for stdin, stout, and stderr.
        """
        TestingLogger.log_cammand(self.hostname, command)
        return self.ssh_client.exec_command(command)
    
    def upload_file(self, file_bytes: IO[bytes], remotepath: str) -> None:
        """
        Uploads a file from the local file path to the remote file path
        on the connected machine.
        """
        self.sfpt_client.putfo(fl=file_bytes, remotepath=remotepath)