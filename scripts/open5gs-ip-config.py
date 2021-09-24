#!/usr/bin/python3

import sys
import yaml


"""
Reconfigure amf and upf with the appropraite ip.
As a side effect, clobbers comments and does not retain order.
"""


def edit_amf(ip: str) -> None:
  print("Configuring amf with '{}'".format(ip))
  with open("/etc/open5gs/amf.yaml", "r") as f:
    config = yaml.safe_load(f)
    config['amf']['ngap'][0]['addr'] = ip
  
  with open("/etc/open5gs/amf.yaml", "w") as f:
    yaml.safe_dump(config, f)

def edit_upf(ip: str):
  print("Configuring upf with '{}'".format(ip))
  with open("/etc/open5gs/upf.yaml", "r") as b:
    config = yaml.safe_load(b)
    config['upf']['gtpu'][0]['addr'] = ip

  with open("/etc/open5gs/upf.yaml", "w") as f:
    yaml.safe_dump(config, f)

def main():
  if len(sys.argv) != 2:
    print("Usage: ./open5gs-ip-config.py <ip>")
    exit(1)

  ip: str = sys.argv[1]
  edit_amf(ip)
  edit_upf(ip)

if __name__ == "__main__":
  main()
