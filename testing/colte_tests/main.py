from colte_tests.nodes.colte_node import ColteNode
from colte_tests.nodes.ueransim_node import UeransimNode


def dev_main():
  """
  Used for informal testing.
  """
  vagrant_dir = "/home/nick/Documents/Workspace/Research/colte-ueransim/vagrant/"

  ueransim = UeransimNode("ueransim", vagrant_dir=vagrant_dir)
  colte1 = ColteNode("colte1", vagrant_dir=vagrant_dir)
  # colte2 = ColteNode("colte2", vagrant_dir=vagrant_dir)

  print("Hello world test:")
  print(" ueransim -", ueransim.run_command("echo hello world"))
  print(" colte1 -", colte1.run_command("echo hello world"))
  # print(" colte2 -", colte2.run_command("echo hello world"))
  print()
  
  print("Device IDs test:")
  print(" ueransim -", ueransim.get_device_ids())


if __name__ == "__main__":
  dev_main()