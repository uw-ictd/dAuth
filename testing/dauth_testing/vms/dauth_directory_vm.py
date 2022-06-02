from typing import Union

from paramiko.channel import ChannelFile

from dauth_testing.vms.vm import VM


class DauthDirectoryVM(VM):
    """
    Represents a dAuth directory VM.
    """

    def __init__(self, vagrant_dir: str, host_name: str) -> None:
        super().__init__(vagrant_dir, host_name)

        self.db_script_path = "~/scripts/open5gs-dbctl"
        self.service_name = "dauth-directory.service"
        self.db_location = "/var/lib/dauth/directory_db.sqlite3"

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

    def get_logs(self) -> ChannelFile:
        """
        Gets the service logs using journalctl.
        Returns stdout and stderr output streams for the log.
        """
        command = " ".join(["sudo", "journalctl", "-fu", self.service_name])
        
        return self.run_input_command(command)[1]
