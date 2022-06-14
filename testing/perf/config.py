import io
from typing import IO

import yaml


class ServiceConfig:
    """
    Represents a config for the dauth service.
    """

    def __init__(self, config_path: str):
        with open(config_path, 'r') as f:
            self.config: dict = yaml.safe_load(f)
        
    def get_file(self) -> IO[bytes]:
        """
        Converts the config to a file IO object.
        """
        return io.StringIO(yaml.safe_dump(self.config, None))

    def add_user(self, imsi: str, sqn_slice_max: dict, backup_network_ids: dict) -> None:
        """
        Add a user to the config.
        """
        self.config["users"][imsi] = {
            "k": "465B5CE8B199B49FAA5F0A2EE238A6BC",
            "opc": "E8ED289DEBA952E4283B54E88E6183CA",
            "sqn_slice_max": sqn_slice_max,
            "backup_network_ids": backup_network_ids
        }
