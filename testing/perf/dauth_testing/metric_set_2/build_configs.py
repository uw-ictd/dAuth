import sys
import os
import yaml
import copy
import random
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
    # set the home network (offline) to the ueransim node
    config_content["services"][0] = copy.deepcopy(config_content["ueransim"])
    config_content["services"][0]["id"] = config_content["services"][0]["id"].replace(
        "-ueransim", "-service"
    )

    other_configs = list()

    for num_backups in (2, 4, 8):
        for random_set in range(5):
            new_config = copy.deepcopy(config_content)

            backups = random.sample(new_config["services"][2:], num_backups)
            new_config["services"] = new_config["services"][0:2]
            new_config["services"].extend(backups)

            other_configs.append(
                (
                    config_name.replace(".yaml", "")
                    + "-nbu{}-rs{}.yaml".format(num_backups, random_set),
                    new_config,
                )
            )

    return other_configs


if __name__ == "__main__":
    build_configs(sys.argv[1], sys.argv[2])
