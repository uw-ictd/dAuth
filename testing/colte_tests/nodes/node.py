import subprocess
from typing import Union

from colte_tests.nodes.command_exception import CommandException


class Node:
  """
  A class representing a single node (VM) of any kind.
  Meant to be subclassed.
  """

  def __init__(self, host_name: str, vagrant_dir :str="./") -> None:
    self.host_name = host_name
    self.vagrant_dir = vagrant_dir
    self.success_stderr = "Connection to"

  def run_command(self, command: str) -> Union[str, str]:
    """
    Runs the provided command in in the home dir of the VM.
    Returns a tuple of the resulting stdout and stderr.
    """
    arg_list = ["vagrant", "ssh", "-c", command, self.host_name]
    proc = subprocess.run(arg_list, capture_output=True, cwd=self.vagrant_dir)

    results = (proc.stdout.decode("utf-8"), proc.stderr.decode("utf-8"))

    if not self.success_stderr in results[1]:
      raise CommandException("Failed to run '{}' on {}, (stdout,stderr): {}'"
          .format(command, self.host_name, results))
    
    return results
