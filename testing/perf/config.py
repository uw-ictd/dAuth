import io
from typing import IO

import yaml
import copy


class Config:
    """
    General yaml-based config.
    """

    def __init__(self, config_path: str):
        with open(config_path, 'r') as f:
            self.config: dict = yaml.safe_load(f)
        
    def get_file(self) -> IO[bytes]:
        """
        Converts the config to a file IO object.
        """
        return io.StringIO(yaml.safe_dump(self.config, None))


class ServiceConfig(Config):
    """
    Represents a config for the dauth service.
    """

    def add_user(self, imsi: str, sqn_slice_max: dict, backup_network_ids: dict) -> None:
        """
        Add a user to the config.
        """
        self.config["users"][imsi] = {
            "k": "465B5CE8B199B49FAA5F0A2EE238A6BC",
            "opc": "E8ED289DEBA952E4283B54E88E6183CA",
            "sqn_slice_max": copy.deepcopy(sqn_slice_max),
            "backup_network_ids": copy.deepcopy(backup_network_ids)
        }
          
    def set_directory_addr(self, addr: str) -> None:
        """
        Sets addr of directory.
        """
        self.config["directory_addr"] = addr
    
    def set_host_addr(self, addr: str) -> None:
        """
        Sets addr to host on.
        """
        self.config["host_addr"] = addr
        
    def set_id(self, id: str) -> None:
        """
        Sets the id.
        """
        self.config["id"] = id
        
    

class UEConfig(Config):
    """
    Represents a config for a UERANSIM UE.
    """
    
    def set_imsi(self, imsi: str) -> None:
        """
        Sets the imsi/supi.
        """
        self.config["supi"] = imsi
    
    def set_gnb_search_list(self, gnb_addresses: list) -> None:
        """
        Sets the gNB search list.
        """
        self.config["gnbSearchList"] = gnb_addresses


class GNBConfig(Config):
    """
    Represents a config for a UERANSIM GNB.
    """
    
    def set_ip(self, ip: str) -> None:
        """
        Sets the ip to host on.
        """
        self.config["linkIp"] = ip
        self.config["ngapIp"] = ip
        self.config["gtpIp"] = ip
        
    def set_nci(self, nci: int) -> None:
        """
        Sets the gNB's NCI value.
        """
        self.config["nci"] = "0x{}0".format(str(nci).zfill(8))
        
    
    def set_amf_addr(self, addr: str) -> None:
        """
        Sets address of amf.
        """
        self.config["amfConfigs"][0]["address"] = addr
