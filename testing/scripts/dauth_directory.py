import argparse
from multiprocessing.pool import ThreadPool

from vms.dauth_directory_vm import DauthDirectoryVM

def print_logs(dauth_directory: DauthDirectoryVM) -> None:
    """
    Prints dauth service logs from the host.
    """
    print(dauth_directory.print_logs())

def stream_logs(dauth_directory: DauthDirectoryVM) -> None:
    """
    Streams dauth service logs as they are created from the host.
    """
    try:
        for line in dauth_directory.streams_logs():
            print(line)
    except KeyboardInterrupt:
        print()
        pass

def start_service(dauth_directory: DauthDirectoryVM) -> None:
    """
    Starts the dauth service if it is not started already.
    """
    print(dauth_directory.start_service())

def stop_service(dauth_directory: DauthDirectoryVM) -> None:
    """
    Stops the dauth service if it is not stopped already.
    """
    print(dauth_directory.stop_service())
    
def remove_state(dauth_directory: DauthDirectoryVM) -> None:
    """
    Removes all local state, including db and keys.
    """
    print(dauth_directory.remove_db())

def reset_service(dauth_directory: DauthDirectoryVM) -> None:
    """
    Resets all of the state of the dauth directory.
    Stops the service, removes state, and starts the service again.
    """
    stop_service(dauth_directory)
    remove_state(dauth_directory)
    start_service(dauth_directory)
    
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
    
    # Specify host name
    parser.add_argument(
        "-n",
        "--host-name",
        required=True,
        help="Vagrant hostnames",
    )
    
    # select one of the following commands
    group = parser.add_mutually_exclusive_group(required=True)
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
        help="Starts the dauth directory service",
    )
    group.add_argument(
        "--stop-service",
        action="store_true",
        help="Stops the dauth directory service",
    )
    group.add_argument(
        "--remove-state",
        action="store_true",
        help="Removes all state for the dauth service",
    )
    group.add_argument(
        "--reset-service",
        action="store_true",
        help="Resets the dauth service directory state completely",
    )

    args = parser.parse_args()
    
    print("Building connection...")
    directory_service = DauthDirectoryVM(args.vagrant_dir, args.host_name)
    
    print("Running command...")
    print(directory_service.host_name + ":")
    if args.print_logs:
        print_logs(directory_service)
    elif args.stream_logs:
        stream_logs(directory_service)
    elif args.start_service:
        start_service(directory_service)
    elif args.stop_service:
        stop_service(directory_service)
    elif args.remove_state:
        remove_state(directory_service)
    elif args.reset_service:
        reset_service(directory_service)
    else:
        print("No action specified")

if __name__ == "__main__":
    main()