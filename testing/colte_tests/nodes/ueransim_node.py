from typing import List, Union

from colte_tests.nodes.node import Node


class UeransimNode(Node):
  """
  Represents the UERANSIM testing node VM.
  """

  def __init__(self, host_name: str, vagrant_dir: str="./",
               cli_path="./UERANSIM/build/nr-cli") -> None:
    super().__init__(host_name, vagrant_dir=vagrant_dir)

    self.cli_path = cli_path
    self.gnbs: List[GNB] = []
    self.ues: List[UE] = []

  def run_cli_command(self, command: str) -> Union[str, str]:
    """
    Runs a command using the cli for ueransim.
    Commands vary by id type (GNB or UE).
    Returns a tuple of stdout and stderr results.
    """
    return self.run_command(" ".join([self.cli_path, command]))

  def get_device_ids(self) -> List[str]:
    """
    Queries all active UERANSIM devices.
    Returns a list of all GNB and UE ids.
    """
    stdout = self.run_cli_command("-d")[0]
    
    return [id.strip() for id in stdout.split("\n") if id.strip() != '']


class DeviceInstance:
  """
  Represents a device instance for UERANSIM.
  May be either a GNB or a UE.
  """

  def __init__(self, node: UeransimNode, id: str) -> None:
    self.node = node
    self.id = id

  def run_device_command(self, command: str) -> Union[str, str]:
    """
    Runs a command on the device.
    Returns a tuple of stdout and stderr results.
    """
    return self.node.run_cli_command(" ".join([self.id, "-e", command]))

  def info(self) -> Union[str, str]:
    """
    Runs info command and returns a tuple of stdout and stderr results.
    """
    return self.run_device_command("info")


class GNB(DeviceInstance):
  """
  Represents a gNodeB instance on the ueransim node.
  """


class UE(DeviceInstance):
  """
  Represents a UE instance on the ueransim node.
  """
