import subprocess
from typing import Union


class Node:
  """
  A class representing a single node (VM) of any kind.
  Meant to be subclassed.
  """

  def __init__(self, host_name: str, vagrant_dir :str="./") -> None:
    self.host_name = host_name
    self.vagrant_dir = vagrant_dir


  def run_command(self, command: str) -> Union[str, str]:
    """
    Runs the provided command in in the home dir of the VM.
    Returns a tuple of the resulting stdout and stderr.
    """
    arg_list = ["vagrant", "ssh", "-c", command, self.host_name]
    proc = subprocess.run(arg_list, capture_output=True, cwd=self.vagrant_dir)
    
    return (proc.stdout.decode("utf-8"), proc.stderr.decode("utf-8"))


if __name__ == "__main__":
  # For testing
  node = Node("ueransim", vagrant_dir="/home/nick/Documents/Workspace/Research/colte-ueransim/vagrant/")
  print(node.run_command("echo hello world"))