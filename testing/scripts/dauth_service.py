import argparse
import io
import subprocess
import os
from typing import List
from multiprocessing.pool import ThreadPool

from connections.service_connection import DauthServiceConnection
from paramiko import SSHConfig


def upload_config(dauth_services: List[DauthServiceConnection], config_path: str) -> None:
    """
    Uploads the config from the provided path.
    Should reset the service after calling this.
    Only uploads a config to the first service in the list.
    """
    service = dauth_services[0]
    with open(config_path, 'r') as f:
        service.upload_config(f)
    
def print_logs(dauth_services: List[DauthServiceConnection]) -> None:
    """
    Prints dauth service logs from the host.
    """
    for dauth_service in dauth_services:
        print(dauth_service.hostname + ":")
        print(dauth_service.print_logs())

def stream_logs(dauth_services: List[DauthServiceConnection]) -> None:
    """
    Prints dauth service logs as they are created from the host.
    """
    for dauth_service in dauth_services:
        print(dauth_service.hostname + ":")
        
        try:
            for line in dauth_service.streams_logs():
                print(line.strip())
        except KeyboardInterrupt:
            print()
            pass

def start_service(dauth_services: List[DauthServiceConnection]) -> None:
    """
    Starts the dauth service if it is not started already.
    """
    for dauth_service in dauth_services:
        print(dauth_service.hostname + ":")
        print(dauth_service.start_service())

def stop_service(dauth_services: List[DauthServiceConnection]) -> None:
    """
    Stops the dauth service if it is not stopped already.
    """
    for dauth_service in dauth_services:
        print(dauth_service.hostname + ":")
        print(dauth_service.stop_service())

def remove_state(dauth_services: List[DauthServiceConnection]) -> None:
    """
    Removes all local state, including db and keys.
    """
    for dauth_service in dauth_services:
        print(dauth_service.hostname + ":")
        print(dauth_service.remove_db())
        print(dauth_service.remove_keys())
    
def reset_service(dauth_services: List[DauthServiceConnection]) -> None:
    """
    Resets all of the state of the dauth service.
    Stops the service, removes state, and starts the service again.
    """
    stop_service(dauth_services)
    remove_state(dauth_services)
    start_service(dauth_services)
    
def ping(dauth_services: List[DauthServiceConnection]) -> None:
    """
    Pings the machine to check for connection.
    """
    for dauth_service in dauth_services:
        print(dauth_service.hostname + 
              ":", "Ping (should say hello) -",
              dauth_service.run_command("echo hello"))

def main():
    parser = argparse.ArgumentParser(
        description='Run commands remotely on a dauth service VM'
    )
    
    # Specifiy vagrant dir if you 
    parser.add_argument(
        "-d",
        "--vagrant-dir",
        required=False,
        help="Vagrantfile directory, specify if connection is to a vagrant VM",
    )
    
    # Specify one or more networks
    parser.add_argument(
        "-n",
        "--host-names",
        nargs='+',
        required=True,
        help="Vagrant hostnames",
    )
    
    # select one of the following commands
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "--upload-config",
        help="Uploads and replaces the service config",
    )
    group.add_argument(
        "--print-logs",
        action="store_true",
        help="Print journalctl logs",
    )
    group.add_argument(
        "--stream-logs",
        action="store_true",
        help="Continuously print journalctl logs",
    )
    group.add_argument(
        "--start-service",
        action="store_true",
        help="Starts the dauth service",
    )
    group.add_argument(
        "--stop-service",
        action="store_true",
        help="Stops the dauth service",
    )
    group.add_argument(
        "--remove-state",
        action="store_true",
        help="Removes all state for the dauth service",
    )
    group.add_argument(
        "--reset-service",
        action="store_true",
        help="Resets the dauth service state completely",
    )
    group.add_argument(
        "--ping",
        action="store_true",
        help="Pings the service to test connection",
    )

    args = parser.parse_args()
    
    # build connections in parallel to save time
    # initial ssh connections take a while
    print("Building connections...")
    
    dauth_services = []

    if args.vagrant_dir:
        vagrant_config = SSHConfig()
        vagrant_config.parse(
            io.StringIO(subprocess.check_output(
                ["vagrant", "ssh-config"], 
                cwd=args.vagrant_dir).decode()
            )
        )
        
        for host_name in args.host_names:
            ssh_info = vagrant_config.lookup(host_name)

            dauth_services.append(DauthServiceConnection(
                ssh_info["hostname"],
                host_name,
                ssh_info["user"],
                int(ssh_info["port"]),
                ssh_info["identityfile"][0]
            ))
    else:
        for host_name in args.host_names:
            dauth_services.append(DauthServiceConnection(
                host_name,
                host_name,
                "ictd",
                22,
                os.path.expanduser("~/.ssh/id_rsa")
            ))
        
        
    print("Running commands...")
    if args.upload_config:
        upload_config(dauth_services, args.upload_config)
    elif args.print_logs:
        print_logs(dauth_services)
    elif args.stream_logs:
        stream_logs(dauth_services)
    elif args.start_service:
        start_service(dauth_services)
    elif args.stop_service:
        stop_service(dauth_services)
    elif args.remove_state:
        remove_state(dauth_services)
    elif args.reset_service:
        reset_service(dauth_services)
    elif args.ping:
        ping(dauth_services)
    else:
        print("No action specified")

if __name__ == "__main__":
    main()