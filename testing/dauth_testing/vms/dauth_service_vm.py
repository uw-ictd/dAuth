from typing import Union

from paramiko.channel import ChannelFile

from dauth_testing.vms.vm import VM


class DauthServiceVM(VM):
    """
    Represents a dAuth service VM.
    """

    def __init__(self, host_name: str, vagrant_dir: str="./") -> None:
        super().__init__(host_name, vagrant_dir=vagrant_dir)

        self.db_script_path = "~/scripts/open5gs-dbctl"
        self.service_name = "dauth.service"
        self.db_location = "/var/lib/dauth/dauth_service/ed25519_keys"
        self.keys_location = "/var/lib/dauth/dauth_service/dauth.sqlite3"

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

    def stream_logs(self) -> ChannelFile:
        """
        Pulls the logs from journalctl.
        Returns stdout and stderr output streams for the log.
        """
        command = " ".join(["sudo", "journalctl", "-fu", self.service_name])
        
        return self.run_input_command(command)[1]
