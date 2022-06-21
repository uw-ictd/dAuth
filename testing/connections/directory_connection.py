from typing import Union

from paramiko.channel import ChannelFile

from connections.connection import Connection


class DauthDirectoryConnection(Connection):
    """
    Represents a dAuth directory connection.
    """

    def __init__(self, hostname: str, id: str, username: str, port: int, keyfile: str) -> None:
        super().__init__(hostname, id, username, port, keyfile)

        self.db_script_path = "~/scripts/open5gs-dbctl"
        self.service_name = "dauth-directory.service"
        self.db_location = "/var/lib/dauth/directory_db.sqlite3"
        
        self.directory_addr = None

    def start_service(self) -> Union[str, str]:
        """
        Calls start on the systemd dauth service.
        Returns the resulting stdout and stderr.
        """
        command = " ".join(["sudo", "systemctl", "start", self.service_name])

        return self.run_command(command)

    def stop_service(self) -> Union[str, str]:
        """
        Calls stop on the systemd dauth service.
        Returns the resulting stdout and stderr.
        """
        command = " ".join(["sudo", "systemctl", "stop", self.service_name])

        return self.run_command(command)

    def remove_db(self) -> Union[str, str]:
        """
        Removes the database for this dauth node.
        The service should be stopped before calling this.
        Returns the resulting stdout and stderr.
        """
        command = " ".join(["sudo", "rm", "-r", self.db_location])

        return self.run_command(command)

    def streams_logs(self) -> ChannelFile:
        """
        Streams the service logs using journalctl.
        Returns stdout and stderr output streams for the log.
        """
        command = " ".join(["sudo", "journalctl", "-fu", self.service_name])
        
        return self.run_input_command(command)[1]
        
    def print_logs(self) -> str:
        """
        Prints the service logs using journalctl.
        Returns stdout and stderr output streams for the log.
        """
        command = " ".join(["sudo", "journalctl", "--no-pager", "-n", "100", "-u", self.service_name])
        
        return self.run_command(command)[0]

    def get_directory_addr(self) -> str:
        """
        Returns the address that the directory service hosts on.
        """
        if self.directory_addr:
            return self.directory_addr
        else:
            return self.hostname + ":8900"
