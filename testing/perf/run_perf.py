import argparse
import logging
from logger import TestingLogger

from perf.state import NetworkState, NetworkStateConfig
from perf.setups.local_auth import LocalAuthSetup
from perf.setups.home_auth import HomeAuthSetup
from perf.setups.backup_auth import BackupAuthSetup


def main():
    TestingLogger.logger.setLevel(logging.INFO)

    parser = argparse.ArgumentParser(
        description='Run the specified perf tests'
    )
    
    parser.add_argument(
        "-d",
        "--vagrant-dir",
        required=True,
        type=str,
        help="Vagrantfile directory",
    )

    parser.add_argument(
        "-c",
        "--config-dir",
        required=True,
        type=str,
        help="Config directory",
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
    
    # select one of the following commands
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
    
    TestingLogger.logger.info("Building state and connecting...")
    config = NetworkStateConfig(args.vagrant_dir, args.config_dir, args.ue_driver)
    state = NetworkState(config)

    if args.local_auth:
        setup = LocalAuthSetup(state)
    elif args.home_auth:
        setup = HomeAuthSetup(state)
    elif args.backup_auth:
        setup = BackupAuthSetup(state)
    else:
        raise Exception("No setup specified")
    
    setup.run_perf(args.num_ues, args.interval, args.iterations)

if __name__ == "__main__":
    main()
