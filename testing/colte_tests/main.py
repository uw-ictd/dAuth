from colte_tests.nodes.colte_node import ColteNode
from colte_tests.nodes.ueransim_node import UeransimNode


def dev_main():
  """
  Used for informal testing.
  """
  vagrant_dir = "/home/nick/Documents/Workspace/Research/colte-ueransim/vagrant/"

  ueransim = UeransimNode("ueransim", vagrant_dir=vagrant_dir)
  colte1 = ColteNode("colte1", vagrant_dir=vagrant_dir)

  print("Hello world test:")
  print(" ueransim -", ueransim.run_command("echo hello world"))
  print(" colte1 -", colte1.run_command("echo hello world"))
  print()

  print("Starting devices")
  gnb = ueransim.add_gnb("./configs/ueransim/gnb-1.yaml", "192.168.40.202")
  ue = ueransim.add_ue("./configs/ueransim/ue.yaml")
  
  print("Device IDs test:")
  print(" ueransim -", ueransim.get_device_ids())
  print()

  print("Device build test")
  print(" GNBs: ", ", ".join([d.id for d in ueransim.gnbs]))
  print(" UEs: ", ", ".join([d.id for d in ueransim.ues]))
  print()

  print("Device info test")
  print(" GNB info\n", ueransim.gnbs[0].info().replace("\n", "\n  "))
  print(" UE info\n", ueransim.ues[0].info().replace("\n", "\n  "))
  print()

  print("Device status test")
  print(" GNB status\n", ueransim.gnbs[0].status().replace("\n", "\n  "))
  print(" UE status\n", ueransim.ues[0].status().replace("\n", "\n  "))
  print()

  print("GNB various commands test")
  print(" amf_list\n", ueransim.gnbs[0].amf_list().replace("\n", "\n  "))
  print(" amf_info\n", ueransim.gnbs[0].amf_info("0").replace("\n", "\n  "))
  print(" amf_info\n", ueransim.gnbs[0].amf_info("2").replace("\n", "\n  "))
  print(" ue_list\n", ueransim.gnbs[0].ue_list().replace("\n", "\n  "))
  print(" ue_count\n", ueransim.gnbs[0].ue_count().replace("\n", "\n  "))
  print(" ue_release\n", ueransim.gnbs[0].ue_release("0").replace("\n", "\n  "))
  print()

  print("UE various commands test")
  print(" timers\n", ueransim.ues[0].timers().replace("\n", "\n  "))
  print(" rls_state\n", ueransim.ues[0].rls_state().replace("\n", "\n  "))
  print(" coverage\n", ueransim.ues[0].coverage().replace("\n", "\n  "))
  print(" ps_establish\n", ueransim.ues[0].ps_establish("sc1 sc2").replace("\n", "\n  "))
  print(" ps_list\n", ueransim.ues[0].ps_list().replace("\n", "\n  "))
  print(" ps_release\n", ueransim.ues[0].ps_release("psid").replace("\n", "\n  "))
  print(" ps_release_all\n", ueransim.ues[0].ps_release_all().replace("\n", "\n  "))
  print(" deregister\n", ueransim.ues[0].deregister("sc").replace("\n", "\n  "))
  print()


if __name__ == "__main__":
  dev_main()