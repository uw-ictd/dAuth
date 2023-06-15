#!/usr/bin/env python3

import argparse
import logging
import subprocess
from typing import List


VANILLA_SERVICES = [

]
    
DAUTH_SERVICES = [

]
    
def init_logger(verbose: bool=False) -> None:
    """
    Performs all setup needed to initialize logging.
    """
    if verbose:
        logging.basicConfig(
            format='[%(asctime)s] %(levelname)s {%(filename)s:%(funcName)s:%(lineno)d} -- %(message)s',
            level=logging.DEBUG)
    else:
        logging.basicConfig(
            format='[%(asctime)s] %(levelname)s {%(filename)s:%(funcName)s:%(lineno)d} -- %(message)s',
            level=logging.INFO)
        

def run_command(command: List[str]) -> None:
    """
    Runs the command in a subprocess/bash.
    Reports errors. Prints all output if verbose is enabled.
    """
    try:
        output = subprocess.check_output(command)
        logging.debug(f"Output:\n{output.decode().strip()}")
    except subprocess.CalledProcessError as e:
        logging.exception(e)
        logging.error(f"Command failed -- {' '.join(command)}")


#### Commands #####


def disable_dauth() -> None:
    """
    Disables all systemd services associated with dauth.
    """
    logging.info("Disabling dauth")
    pass


def enable_dauth() -> None:
    """
    Enables all systemd services associated with dauth.
    """
    logging.info("Enabling dauth")
    pass


def disable_vanilla() -> None:
    """
    Disables all systemd services associated with vanilla open5gs.
    """
    logging.info("Disabling vanilla open5gs")
    pass


def enable_vanilla() -> None:
    """
    Enables all systemd services associated with vanilla open5gs.
    """
    logging.info("Enabling vanilla open5gs")
    pass


def sync_dauth() -> None:
    """
    Updates dauth with any new user state from vanilla open5gs.
    """
    logging.info("Syncing dauth with vanilla open5gs")
    pass


def sync_vanilla() -> None:
    """
    Updates vanilla open5gs with any new user state from dauth.
    """
    logging.info("Syncing vanilla open5gs with dauth")
    pass


def switch_to_dauth() -> None:
    """
    Disables vanilla open5gs, transfers state, and enables dauth.
    """
    logging.info("Swithing to dauth")
    disable_vanilla()
    sync_dauth()
    enable_dauth()


def switch_to_vanilla() -> None:
    """
    Disables dauth, transfers state, and enables vanilla open5gs.
    """
    logging.info("Switching to vanilla open5gs")
    disable_dauth()
    sync_vanilla()
    enable_vanilla()


def add_user() -> None:
    """
    Attempts to add a new user to both vanilla open5gs and dauth.
    """
    logging.info("Adding user")
    pass


def main() -> None:
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--verbose",
        help="Enables verbose (debug) logging",
        action="store_true"
    )

    sp = parser.add_subparsers(help="Type of command")
    manage = sp.add_parser("manage", help="Manages running instances of vanilla open5gs and dauth")
    add = sp.add_parser("add-user", help="Attempts to add a new user to both vanilla open5gs and dauth")

    command = manage.add_mutually_exclusive_group(required=True)

    command.add_argument(
        "--disable-dauth",
        help="Disables all systemd services associated with dauth",
        action="store_true"
    )

    command.add_argument(
        "--enable-dauth",
        help="Enables all systemd services associated with dauth",
        action="store_true"
    )

    command.add_argument(
        "--disable-vanilla",
        help="Disables all systemd services associated with vanilla open5gs",
        action="store_true"
    )

    command.add_argument(
        "--enable-vanilla",
        help="Enables all systemd services associated with vanilla open5gs",
        action="store_true"
    )

    command.add_argument(
        "--sync-dauth",
        help="Updates dauth with any new user state from vanilla open5gs",
        action="store_true"
    )

    command.add_argument(
        "--sync-vanilla",
        help="Updates vanilla open5gs with any new user state from dauth",
        action="store_true"
    )

    command.add_argument(
        "--switch-dauth",
        help="Disables vanilla open5gs, transfers state, and enables dauth",
        action="store_true"
    )

    command.add_argument(
        "--switch-vanilla",
        help="Disables dauth, transfers state, and enables vanilla open5gs",
        action="store_true"
    )

    add.add_argument("imsi")
    add.add_argument("k")
    add.add_argument("opc")
    
    args = parser.parse_args()
    init_logger(args.verbose)

    logging.info("Running dauth management script")

    if args.disable_dauth:
        disable_dauth()
    elif args.enable_dauth:
        enable_dauth()
    elif args.disable_vanilla:
        disable_vanilla()
    elif args.enable_vanilla:
        enable_vanilla()
    elif args.sync_dauth:
        sync_dauth()
    elif args.sync_vanilla:
        sync_vanilla()
    elif args.switch_dauth:
        switch_to_dauth()
    elif args.switch_vanilla:
        switch_to_vanilla()
    else:
        logging.error("No valid command selected")

if __name__ == "__main__":
    main()
