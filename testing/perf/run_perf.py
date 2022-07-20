import argparse
import logging
import io
import os
import subprocess

from paramiko import SSHConfig

from logger import TestingLogger
from perf.state import NetworkState, NetworkStateConfig, ConnectionConfig
from perf.setups.local_auth import LocalAuthSetup
from perf.setups.home_auth import HomeAuthSetup
from perf.setups.backup_auth import BackupAuthSetup
import yaml


def main():

    parser = argparse.ArgumentParser(description="Run the specified perf tests")

    parser.add_argument(
        "-p",
        "--network-config",
        required=True,
        type=str,
        help="Path to config for the network, i.e. hostnames",
    )

    parser.add_argument(
        "-d",
        "--vagrant-dir",
        required=False,
        type=str,
        help="Optional vagrantfile directory",
    )

    parser.add_argument(
        "-c",
        "--config-dir",
        required=True,
        type=str,
        help="Config directory for perf device/service configs",
    )

    parser.add_argument(
        "-u",
        "--ue-driver",
        required=True,
        type=str,
        help="UE driver file path",
    )

    parser.add_argument(
        "-n",
        "--num-ues",
        required=True,
        type=int,
        help="Number of UEs",
    )

    parser.add_argument(
        "-i",
        "--interval",
        required=True,
        type=int,
        help="Interval in milliseconds to connect and reconnect",
    )

    parser.add_argument(
        "-t",
        "--iterations",
        required=True,
        type=int,
        help="Number of times to reconnect",
    )

    parser.add_argument(
        "-k",
        "--key-threshold",
        required=False,
        type=int,
        help="Set key threshold for backup auth",
    )

    parser.add_argument(
        "--debug",
        action="store_true",
        help="Change logging level to debug",
    )

    # select one of the following setups
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "--local-auth",
        action="store_true",
        help="Configure the network for local auth",
    )
    group.add_argument(
        "--home-auth",
        action="store_true",
        help="Configure the network for home auth",
    )
    group.add_argument(
        "--backup-auth",
        action="store_true",
        help="Configure the network for backup auth",
    )

    args = parser.parse_args()

    if args.num_ues <= 10:
        # Increase the number of samples but at the same steady-state rate
        args.num_ues = args.num_ues * 5
        args.interval = args.interval * 5
    elif args.num_ues < 100:
        # Increase the number of samples but at the same steady-state rate
        args.num_ues = args.num_ues * 2
        args.interval = args.interval * 2

    if args.debug:
        TestingLogger.logger.setLevel(logging.DEBUG)
    else:
        TestingLogger.logger.setLevel(logging.INFO)

    TestingLogger.logger.info("Building state and connecting...")
    config = NetworkStateConfig(args.config_dir, args.ue_driver)

    build_config(
        config, yaml.safe_load(open(args.network_config, "r")), args.vagrant_dir
    )

    state = NetworkState(config)

    if args.local_auth:
        setup = LocalAuthSetup(state)
    elif args.home_auth:
        setup = HomeAuthSetup(state)
    elif args.backup_auth:
        setup = BackupAuthSetup(state)
    else:
        raise Exception("No setup specified")

    if args.key_threshold:
        setup.key_threshold = args.key_threshold

    setup.run_perf(args.num_ues, args.interval, args.iterations)


def build_config(config: NetworkStateConfig, network: dict, vagrant_dir: str) -> None:
    vagrant_config = None
    if vagrant_dir:
        vagrant_config = SSHConfig()
        vagrant_config.parse(
            io.StringIO(
                subprocess.check_output(
                    ["vagrant", "ssh-config"], cwd=vagrant_dir
                ).decode()
            )
        )

    config.directory_config = handle_connection(network["directory"], vagrant_config)
    config.ueransim_config = handle_connection(network["ueransim"], vagrant_config)
    for service in network["services"]:
        config.service_configs.append(handle_connection(service, vagrant_config))


def handle_connection(
    connection_info: dict, vagrant_config: SSHConfig
) -> ConnectionConfig:
    if connection_info["is_vagrant"]:
        if not vagrant_config:
            raise Exception("No vagrant dir specified")

        ssh_info = vagrant_config.lookup(connection_info["hostname"])

        conf = ConnectionConfig(
            ssh_info["hostname"],
            connection_info["id"],
            ssh_info["user"],
            int(ssh_info["port"]),
            ssh_info["identityfile"][0],
        )

        conf.service_ip = connection_info.get("service_ip")
        conf.directory_addr = connection_info.get("directory_addr")
        return conf
    else:
        return ConnectionConfig(
            connection_info["hostname"],
            connection_info["id"],
            "ictd",  # TODO: look into this,
            22,
            os.path.expanduser("~/.ssh/id_rsa"),
        )


if __name__ == "__main__":
    main()
