from colte_tests.nodes.node import Node


class CoreNode(Node):
  """
  Represents the UERANSIM testing node VM.
  """

  def __init__(self, host_name: str, vagrant_dir: str="./") -> None:
    super().__init__(host_name, vagrant_dir=vagrant_dir)

  # TODO: Implement testing functionality
  # - Implement subclasses/objects for UEs and gNodeBs
  # - OR have the class keep track of it (maybe have these anyway?)
