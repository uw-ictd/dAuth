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
    Finds all active UERANSIM devices.
    Returns a list of all GNB and UE ids.
    """
    stdout = self.run_cli_command("-d")[0]
    
    return [id.strip() for id in stdout.split("\n") if id.strip() != '']

  def detect_devices(self) -> None:
    """
    Finds active UERANSIM devices and creates objects representing them.
    Replaces existing lists.
    """
    device_ids = self.get_device_ids()

    self.gnbs.clear()
    self.ues.clear()

    for device_id in device_ids:
      if device_id.startswith("imsi-"):
        self.ues.append(UE(self, device_id))
      else:
        self.gnbs.append(GNB(self, device_id))


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
    return self.node.run_cli_command(" ".join([self.id, "-e", "\"{}\"".format(command)]))

  def info(self) -> str:
    """
    Runs info command and returns the results.
    Help: 'Show some information about the UE/gNB'
    """
    return self.run_device_command("info")[0]

  def status(self) -> str:
    """
    Runs status command and returns the results.
    Help: 'Show some status information about the UE/gNB'
    """
    return self.run_device_command("status")[0]


class GNB(DeviceInstance):
  """
  Represents a gNodeB instance on the ueransim node.
  """

  def amf_list(self) -> str:
    """
    Runs amf-list command and returns the results.
    Help: 'List all AMFs associated with the gNB'
    """
    return self.run_device_command("amf-list")[0]

  def amf_info(self, amf_id: str) -> str:
    """
    Runs amf-info command on the provided amf_id and returns the results.
    Help: 'Show some status information about the given AMF'
    """
    return self.run_device_command(" ".join(["amf-info", amf_id]))[0]

  def ue_list(self) -> str:
    """
    Runs ue-list command and returns the results.
    Help: 'List all UEs associated with the gNB'
    """
    return self.run_device_command("ue-list")[0]

  def ue_count(self) -> str:
    """
    Runs ue-count command and returns the results.
    Help: 'Print the total number of UEs connected the this gNB'
    """
    return self.run_device_command("ue-count")[0]

  def ue_release(self, ue_id: str) -> str:
    """
    Runs ue-release command on the provided ue_id and returns the results.
    Help: 'Request a UE context release for the given UE'
    """
    return self.run_device_command(" ".join(["ue-release", ue_id]))[0]


class UE(DeviceInstance):
  """
  Represents a UE instance on the ueransim node.
  """

  def timers(self) -> str:
    """
    Runs timers command and returns the results.
    Help: 'Dump current status of the timers in the UE'
    """
    return self.run_device_command("timers")[0]

  def rls_state(self) -> str:
    """
    Runs rls-state command and returns the results.
    Help: 'Show status information about RLS'
    """
    return self.run_device_command("rls-state")[0]

  def coverage(self) -> str:
    """
    Runs coverage command and returns the results.
    Help: 'Dump available cells and PLMNs in the coverage'
    """
    return self.run_device_command("coverage")[0]

  def ps_establish(self, subcommand: str) -> str:
    """
    Runs ps-establish command with the provided subcommand and returns the results.
    Help: 'Trigger a PDU session establishment procedure'
    """
    return self.run_device_command(" ".join(["ps-establish", subcommand]))[0]

  def ps_list(self) -> str:
    """
    Runs ps-list command and returns the results.
    Help: 'List all PDU sessions'
    """
    return self.run_device_command("ps-list")[0]

  def ps_release(self, ps_id: str) -> str:
    """
    Runs ps-release command with the provided ps_id and returns the results.
    Help: 'Trigger a PDU session release procedure'
    """
    return self.run_device_command(" ".join(["ps-release", ps_id]))[0]

  def ps_release_all(self) -> str:
    """
    Runs ps-release-all command and returns the results.
    Help: 'Trigger PDU session release procedures for all active sessions'
    """
    return self.run_device_command("ps-release-all")[0]

  def deregister(self, subcommand: str) -> str:
    """
    Runs deregister command with the provided subcommand and returns the results.
    Help: 'Perform a de-registration by the UE'
    """
    return self.run_device_command(" ".join(["deregister", subcommand]))[0]
