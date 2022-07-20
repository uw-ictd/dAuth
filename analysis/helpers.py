backup_metadata_extraction_regex = re.compile(r"^backup_auth:<H,S,B,T>\(([A-Z,a-z,\-]+),([A-Z,a-z,\-]+),(\[.+\]),([0-9]+)\):<n,i,t>\(([0-9]+),([0-9]+),([0-9]+)\)$")

def extract_metadata_from_backup_test_name(name_string: str) -> dict[str, str]:
    matches = backup_metadata_extraction_regex.fullmatch(name_string)
    if len(matches.groups()) != 7:
        log.error("Could not parse: %s", name_string)
        raise ValueError("Invalid test name parsed")

    result_groups = matches.groups()
    backup_networks_string = result_groups[2]
    backup_networks_string = backup_networks_string.replace("[", "")
    backup_networks_string = backup_networks_string.replace("]", "")
    backup_networks = backup_networks_string.split(",")

    trimmed_network_list = []
    for net in backup_networks:
        trimmed_network_list.append(net.replace("'", "").strip())

    res = {
        "home_network": result_groups[0],
        "serving_network": result_groups[1],
        "backup_networks": trimmed_network_list,
        "threshold": int(result_groups[3]),
        "ue_count": int(result_groups[4]),
        "target_time_ms": int(result_groups[5]),
        "backup_count": len(trimmed_network_list)
    }
    log.debug("Parsed test metadata: %s", res)

    return res
