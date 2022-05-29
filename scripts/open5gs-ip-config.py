#!/usr/bin/python3

import argparse
import sys
import yaml

from pathlib import Path


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

def read_host_config(config_path=Path("/etc/dauth/host-config.yaml")):
  with open(config_path) as f:
    host_config = yaml.safe_load(f)

  return host_config

def main():
  parser = argparse.ArgumentParser(
    description="Update bind ip addresses in the open5gs configuration"
  )
  parser.add_argument(
    "bind_ip",
    nargs="?",
    default=None,
    type=str,
    help="The ip address written into the config files",
  )

  args = parser.parse_args()

  if args.bind_ip is not None:
      ip: str = args.parse_ip
  else:
    print("No IP provided via command line, reading from config")
    host_config = read_host_config()
    ip: str = host_config["public-ip"]

  edit_amf(ip)
  edit_upf(ip)

if __name__ == "__main__":
  main()
