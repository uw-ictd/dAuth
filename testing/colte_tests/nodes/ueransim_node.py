import yaml
from os import path
from typing import List, Set, Union
from paramiko.client import SSHClient

from colte_tests.nodes.command_exception import CommandException
from colte_tests.nodes.node import Node
from colte_tests.logger import TestingLogger


class UeransimNode(Node):
  """
  Represents the UERANSIM testing node VM.
  """

  def __init__(self, host_name: str, vagrant_dir: str="./",
               build_path: str="./UERANSIM/build/") -> None:
    super().__init__(host_name, vagrant_dir=vagrant_dir)

    self.build_path: str = build_path
    self.gnbs: List[GNB] = []
    self.ues: List[UE] = []

  def run_cli_command(self, command: str) -> Union[str, str]:
    """
    Runs a command using the cli for ueransim.
    Commands vary by id type (GNB or UE).
    Returns a tuple of stdout and stderr results.
    """
    return self.run_command(" ".join([path.join(self.build_path, "nr-cli"), command]))

  def get_device_ids(self) -> Set[str]:
    """
    Finds all active UERANSIM devices.
    Returns a set of all GNB and UE ids.
    """
    stdout = self.run_cli_command("-d")[0]
    
    return set([id.strip() for id in stdout.split("\n") if id.strip() != ''])

  def add_gnb(self, config_path: str, ip: str) -> "GNB":
    """
    Builds and starts a GNB device.
    Returns the resulting GNB object.
    """

    gnb = GNB(self, config_path, ip)
    gnb.start_device()
    self.gnbs.append(gnb)
    return gnb

  def add_ue(self, config_path: str) -> "UE":
    """
    Builds and starts a GNB device.
    Returns the resulting GNB object.
    """

    ue = UE(self, config_path)
    ue.start_device()
    self.ues.append(ue)
    return ue


class DeviceInstance:
  """
  Represents a device instance for UERANSIM.
  May be either a GNB or a UE.
  """

  def __init__(self, node: UeransimNode, config_path: str) -> None:
    self.node: UeransimNode = node
    self.config_path: str = config_path
  
    self.id: str = None
    self.device_type: str = None  # Set when subclassed
    self.connection: SSHClient = None

  def start_device(self) -> None:
    """
    Starts the device and builds a connection.
    Adds the ip address on the VM.
    """
    if self.connection is None:
      self.connection = self.node.build_ssh_client()

      self.startup_tasks()
      self.generate_id()

      self.connection.exec_command("{} -c {}".format(
        path.join(self.node.build_path, self.device_type), 
        self.config_path), get_pty=True)[0]

  def startup_tasks(self) -> None:
    """
    Specific device startup tasks to be run during "start_device".
    """
    pass

  def generate_id(self) -> None:
    """
    Specific device id generation to be run during "start_device".
    Sets the id.
    """
    self.id = None

  def stop_device(self) -> None:
    """
    Stops the device and kills the connection.
    """
    if self.connection is not None:
      self.connection.close()
      self.connection = None

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

  def __init__(self, node: UeransimNode, config_path: str, ip: str) -> None:
    super().__init__(node, config_path)
    self.ip: str = ip
    self.device_type: str = "nr-gnb"

  def startup_tasks(self) -> None:
    """
    Specific device startup tasks to be run during "start_device".
    """
    command = "sudo ip addr add {} dev enp0s8".format(self.ip)
    TestingLogger.log_command_streams(command, self.connection.exec_command(command))

    # Note: Using config_path as the base config
    config_producer = path.join(path.dirname(self.config_path), "gnb_config_producer.py")
    new_config = "".join([self.config_path.replace(".yaml", ""), "-", 
      self.ip.replace(".", "_"), ".yaml"])

    # Build the new config and set it as the current
    command = " ".join(["sudo", config_producer, self.config_path, self.ip, new_config])
    TestingLogger.log_command_streams(command, self.connection.exec_command(command))
    self.config_path = new_config

  def generate_id(self) -> None:
    """
    Specific device id generation to be run during "start_device".
    Sets the id.
    """
    content = yaml.safe_load(self.node.run_command("cat {}".format(self.config_path))[0])
    self.id = "UERANSIM-gnb-{}-{}-{}".format(content['mcc'], 
      content['mnc'], int(content['nci'][2:-1], base=16))

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

  def __init__(self, node: UeransimNode, config_path: str) -> None:
    super().__init__(node, config_path)
    self.device_type: str = "nr-ue"

  def generate_id(self) -> None:
    """
    Specific device id generation to be run during "start_device".
    Sets the id.
    """
    content = yaml.safe_load(self.node.run_command("cat {}".format(self.config_path))[0])
    self.id = content['supi']

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
