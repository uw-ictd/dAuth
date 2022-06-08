import subprocess
import io
from typing import IO, Union

from paramiko import SFTPClient, SSHConfig, SSHClient, AutoAddPolicy
from paramiko.channel import ChannelFile, ChannelStderrFile, ChannelStdinFile

from logger import TestingLogger


class VM:
    """
    A class representing a single node VM of any kind. Meant to be subclassed.
    Uses paramiko to maintain ssh connection, connects on build.
    """

    def __init__(self, vagrant_dir: str, host_name: str) -> None:
        self.host_name = host_name
        self.vagrant_dir = vagrant_dir

        # Initialize paramiko connection and make main connection
        config = SSHConfig()
        config.parse(
            io.StringIO(
                subprocess.check_output(
                    ["vagrant", "ssh-config", host_name], 
                    cwd=vagrant_dir).decode()))
        self.ssh_info = config.lookup(host_name)
        self.ssh_client: SSHClient = self.build_ssh_client()
        self.sfpt_client: SFTPClient = self.ssh_client.open_sftp()

    def build_ssh_client(self) -> SSHClient:
        """
        Builds and returns an ssh client to the node.
        """
        ssh_client = SSHClient()
        ssh_client.set_missing_host_key_policy(AutoAddPolicy())
        ssh_client.connect(self.ssh_info["hostname"],
            port=int(self.ssh_info["port"]),
            username=self.ssh_info["user"],
            key_filename=self.ssh_info["identityfile"][0],
            timeout=30)
        
        return ssh_client

    def run_command(self, command: str) -> Union[str, str]:
        """
        Runs the provided command in in the home dir of the VM.
        Returns a tuple of the resulting stdout and stderr.
        """
        outputs = self.run_input_command(command)
        stdout, stderr = outputs[1].read().decode(), outputs[2].read().decode()
        return (stdout, stderr)

    def run_input_command(self, command: str) -> Union[ChannelStdinFile, ChannelFile, ChannelStderrFile]:
        """
        Runs the provided command in in the home dir of the VM.
        Returns active streams for stdin, stout, and stderr.
        """
        TestingLogger.log_cammand(self.host_name, command)
        return self.ssh_client.exec_command(command)
    
    def upload_file(self, file_bytes: IO[bytes], remotepath: str) -> None:
        """
        Uploads a file from the local file path to the remote file path
        on the VM.
        """
        self.sfpt_client.putfo(fl=file_bytes, remotepath=remotepath)