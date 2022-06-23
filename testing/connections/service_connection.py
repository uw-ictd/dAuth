from typing import IO, Union

from paramiko.channel import ChannelFile
from tests.config import ServiceConfig

from connections.connection import Connection


class DauthServiceConnection(Connection):
    """
    Represents a dAuth service connection.
    """

    def __init__(self, hostname: str, id: str, username: str, port: int, keyfile: str) -> None:
        super().__init__(hostname, id, username, port, keyfile)

        self.db_script_path = "~/scripts/open5gs-dbctl"
        self.service_name = "dauth.service"
        self.db_location = "/var/lib/dauth/dauth_service/ed25519_keys"
        self.keys_location = "/var/lib/dauth/dauth_service/dauth.sqlite3"
        self.config_location = "/etc/dauth/dauth.yaml"
        self.tmp_config_location = "/tmp/dauth.yaml"
        
        self.service_ip: str = None
        
    def upload_config(self, file: IO[bytes]):
        """
        Uploads the config file to the service vm.
        Should reset the service state after calling this.
        """
        self.upload_file(file, self.tmp_config_location)
        
        # sftp does not have root permissions, so we need to store in a temp location
        # then move the uploaded file using ssh root permissions
        self.run_command(
            " ".join(["sudo", "mv", self.tmp_config_location, self.config_location]))
        
    def change_config(self, config: ServiceConfig):
        """
        Changes the main config to the provided config.
        Should reset the service state after calling this.
        """
        self.upload_config(config.get_file())

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

    def remove_keys(self) -> Union[str, str]:
        """
        Removes the keys for this dauth node.
        The service should be stopped before calling this.
        Returns the resulting stdout and stderr.
        """
        command = " ".join(["sudo", "rm", "-r", self.keys_location])

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
    
    def get_amf_ip(self) -> str:
        """
        Returns the amf ip.
        """

        if self.service_ip:
            return self.service_ip
        else:
            return self.hostname
        
    def get_host_addr(self) -> str:
        """
        Returns the address dauth hosts on
        """
        if self.service_ip:
            return self.service_ip + ":50051"
        else:
            return self.hostname + ":50051"