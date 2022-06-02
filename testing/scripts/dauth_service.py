import argparse
from typing import List
from multiprocessing.pool import ThreadPool

from vms.dauth_service_vm import DauthServiceVM

def print_logs(dauth_services: List[DauthServiceVM]) -> None:
    """
    Prints dauth service logs as they are created from the host.
    """
    for dauth_service in dauth_services:
        print(dauth_service.host_name + ":")
        
        try:
            for line in dauth_service.get_logs():
                print(line)
        except KeyboardInterrupt:
            print()
            pass

def start_service(dauth_services: List[DauthServiceVM]) -> None:
    """
    Starts the dauth service if it is not started already.
    """
    for dauth_service in dauth_services:
        print(dauth_service.host_name + ":")
        print(dauth_service.start_service())

def stop_service(dauth_services: List[DauthServiceVM]) -> None:
    """
    Stops the dauth service if it is not stopped already.
    """
    for dauth_service in dauth_services:
        print(dauth_service.host_name + ":")
        print(dauth_service.stop_service())

def remove_state(dauth_services: List[DauthServiceVM]) -> None:
    """
    Removes all local state, including db and keys.
    """
    for dauth_service in dauth_services:
        print(dauth_service.host_name + ":")
        print(dauth_service.remove_db())
        print(dauth_service.remove_keys())
    
def reset_service(dauth_services: List[DauthServiceVM]) -> None:
    """
    Resets all of the state of the dauth service.
    Stops the service, removes state, and starts the service again.
    """
    stop_service(dauth_services)
    remove_state(dauth_services)
    start_service(dauth_services)
    
def build_vm(vagrant_dir: str, host_name: str) -> DauthServiceVM:
    """
    Builds a vm object.
    """
    return DauthServiceVM(vagrant_dir, host_name)

def main():
    parser = argparse.ArgumentParser(
        description='Run commands remotely on a dauth service VM'
    )
    
    # must specify the vagrant dir
    parser.add_argument(
        "-d",
        "--vagrant-dir",
        required=True,
        help="Vagrantfile directory",
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
        "--print-logs",
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

    args = parser.parse_args()
    
    # build connections in parallel to save time
    # initial ssh connections take a while
    print("Building connections...")
    pool = ThreadPool()
    results = []
    for host_name in args.host_names:
        results.append(pool.apply_async(build_vm, (args.vagrant_dir, host_name)))
    
    dauth_services = []
    for result in results:
        dauth_services.append(result.get())
        
    print("Running commands...")
    if args.print_logs:
        print_logs(dauth_services)
    elif args.start_service:
        start_service(dauth_services)
    elif args.stop_service:
        stop_service(dauth_services)
    elif args.remove_state:
        remove_state(dauth_services)
    elif args.reset_service:
        reset_service(dauth_services)
    else:
        print("No action specified")

if __name__ == "__main__":
    main()