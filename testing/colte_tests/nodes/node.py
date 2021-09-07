import subprocess
import io

from paramiko import SSHConfig, SSHClient, AutoAddPolicy
from typing import Union

from colte_tests.nodes.command_exception import CommandException


class Node:
  """
  A class representing a single node (VM) of any kind. Meant to be subclassed.
  Uses paramiko to maintain ssh connection, connects on build.
  """

  def __init__(self, host_name: str, vagrant_dir: str="./") -> None:
    self.host_name = host_name
    self.vagrant_dir = vagrant_dir

    # Initialize paramiko connection
    config = SSHConfig()
    config.parse(
      io.StringIO(
        subprocess.check_output(
          ["vagrant", "ssh-config", host_name], 
          cwd=vagrant_dir).decode()))
    self.ssh_info = config.lookup(host_name)
    self.ssh_client = SSHClient()
    self.ssh_client.set_missing_host_key_policy(AutoAddPolicy())
    self.ssh_client.connect(self.ssh_info["hostname"],
      port=int(self.ssh_info["port"]),
      username=self.ssh_info["user"],
      key_filename=self.ssh_info["identityfile"][0],
      timeout=30)

  def run_command(self, command: str) -> Union[str, str]:
    """
    Runs the provided command in in the home dir of the VM.
    Returns a tuple of the resulting stdout and stderr.
    """
    outputs = self.ssh_client.exec_command(command)
    return (outputs[1].read().decode(), outputs[2].read().decode())
