import sys
import os
import yaml
import copy
from typing import List, Union


def build_configs(config_dir: str, output_dir: str):
    for config_name in os.listdir(config_dir):
        config_content = yaml.safe_load(
            open(os.path.join(config_dir, config_name), "r")
        )
        other_configs = generate_variations(config_name, config_content)

        for (filename, content) in other_configs:
            filepath = os.path.join(output_dir, filename)
            with open(filepath, "w") as of:
                of.write(yaml.dump(content))


def generate_variations(
    config_name: str, config_content: dict
) -> List[Union[str, dict]]:
    backups = config_content["services"][2:]

    other_configs = list()
    for backup in copy.deepcopy(backups):
        new_config = copy.deepcopy(config_content)
        new_config["services"].remove(backup)
        new_config["services"][0] = backup

        other_configs.append(
            (
                config_name.replace(".yaml", "")
                + "-"
                + backup["id"].replace("-service", ".yaml"),
                new_config,
            )
        )

    return other_configs


if __name__ == "__main__":
    build_configs(sys.argv[1], sys.argv[2])
