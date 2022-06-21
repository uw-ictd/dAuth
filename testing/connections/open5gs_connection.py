from typing import Union

from connections.connection import Connection


class Open5gsConnection(Connection):
    """
    Represents a colte core connection.
    """

    def __init__(self, hostname: str, id: str, username: str, port: int, keyfile: str) -> None:
        super().__init__(hostname, id, username, port, keyfile)

        self.db_script_path = "~/scripts/open5gs-dbctl"

    def add_subscriber(self, imsi: str, key: str, opc: str, ip: str=None) -> Union[str, str]:
        """
        Adds a subscriber to the db.
        Returns the result of executing the db change script.
        """
        if ip is not None:
            command = " ".join([self.db_script_path, "add", imsi, ip, key, opc])
        else:
            command = " ".join([self.db_script_path, "add", imsi, key, opc])

        return self.run_command(command)

    def remove_subscriber(self, imsi: str) -> Union[str, str]:
        """
        Removes a subscriber from the db.
        Returns the result of executing the db change script.
        """
        command = " ".join([self.db_script_path, "remove", imsi])

        return self.run_command(command)

    def reset(self) -> Union[str, str]:
        """
        Resets the db, removing everything.
        Returns the result of executing the db change script.
        """
        command = " ".join([self.db_script_path, "reset"])

        return self.run_command(command)
