#!/usr/bin/python3

import sys
import yaml
import os

if len(sys.argv) != 4:
  print("Usage: ./config_producer <config base> <ip> <new config>")
  exit(1)

base: str = sys.argv[1]
ip: str = sys.argv[2]
new: str = sys.argv[3]

if not os.path.exists(base):
  print("Base config does not exist")
  exit(1)

# Sanity check ip?

if os.path.exists(new):
  print("Config already generated, skipping")
else:
  with open(base, "r") as b:
    config = yaml.safe_load(b)
    config["linkIp"] = ip
    config["ngapIp"] = ip
    config["gtpIp"] = ip

    with open(new, "w") as n:
      yaml.safe_dump(config, n)
  
exit(0)
