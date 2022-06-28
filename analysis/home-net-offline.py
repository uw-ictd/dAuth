from collections import defaultdict
from pathlib import Path
import json
import logging
import re

import altair as alt
import pandas as pd

# Module specific format options
pd.set_option('display.max_columns', None)
pd.set_option('display.max_colwidth', None)
pd.set_option('display.width', None)
pd.set_option('display.max_rows', 40)

logging.basicConfig()

log = logging.getLogger(__name__)
log.setLevel(logging.DEBUG)

backup_metadata_extraction_regex = re.compile(r"^backup_auth:<H,S,B,T>\(([A-Z,a-z,\-]+),([A-Z,a-z,\-]+),(\[.+\])\):<n,i,t>\(([0-9]+),([0-9]+),([0-9]+)\)$")
filename_metadata_extraction_regex = re.compile(r"^([0-9]+)-nbu[0-9]+-rs[0-9]+.out$")
user_sim_number = re.compile(r"^90170([0-9]+)$")

def normalize_json_to_dataframe(result_directory_path: Path):
    result_filenames = result_directory_path.glob("*")
    result_filenames = list(result_filenames)
    result_filenames.sort()

    datapoints = []
    drop_count = 0
    drop_counters_per_backup_network = defaultdict(lambda: 0)
    backup_network_inclusion_frequencies = defaultdict(lambda: 0)
    drop_counters_per_num_ues = defaultdict(lambda: 0)
    for filename in result_filenames:
        scenario = extract_metadata_from_filename(filename.name)
        with open(filename) as f:
            lines = []
            for line in f:
                lines.append(line)

            # Keep track of appearances of the accidentally duplicated names to identify the thresholds used in a particular test
            name_appearance_count = defaultdict(lambda: 0)

            for i, line in enumerate(lines):
                parsed_json = json.loads(line)

                # See if the line is a high-level result line based on the presence of the test_name key
                try:
                    test_parameters = extract_metadata_from_test_name(parsed_json["test_name"])
                except KeyError:
                    # The line is a tokio timing line we're not using for now
                    pass
                    continue

                test_parameters["total_test_duration_s"] = float(parsed_json["test_duration"])
                test_parameters["total_test_auth_count"] = int(parsed_json["total_auths"])
                test_parameters["scenario"] = scenario

                name_appearance_count[parsed_json["test_name"]] += 1

                if name_appearance_count[parsed_json["test_name"]] == 1:
                    test_parameters["threshold"] = 2
                elif name_appearance_count[parsed_json["test_name"]] == 2:
                    test_parameters["threshold"] = 4
                elif name_appearance_count[parsed_json["test_name"]] == 3:
                    test_parameters["threshold"] = 8
                else:
                    raise ValueError("Unknown threshold encountered, results possibly corrupt")

                # Attempt to parse timing data from each provided UE.
                found_ues = 0
                for key in parsed_json.keys():
                    try:
                        user_number = extract_ue_number_from_imsi(key)
                        found_ues += 1
                    except ValueError:
                        # Continue looking at other keys if this key is not a valid imsi
                        continue

                    ue_results = parsed_json[key]["results"]
                    for iteration in zip(ue_results["nanoseconds_since_auth"], ue_results["nanoseconds_since_registration"], ue_results["nanoseconds_to_establish_session"]):
                        datapoint = {
                            "user_id": user_number,
                            "auth_ns": iteration[0],
                            "registration_ns": iteration[1],
                            "session_ns": iteration[2],
                            }
                        datapoint = datapoint | test_parameters
                        datapoints.append(datapoint)

                if found_ues == 0:
                    log.warning("Test failed to execute and has no returned results, dropping from analysis: %s", parsed_json["test_name"])
                    drop_count += 1
                    drop_counters_per_num_ues[test_parameters["total_test_auth_count"]] += 1
                    for net in test_parameters["backup_networks"]:
                        drop_counters_per_backup_network[net] += 1

                for net in test_parameters["backup_networks"]:
                    backup_network_inclusion_frequencies[net] += 1

    print(drop_count)

    backup_drop_ratios = dict()
    for net in backup_network_inclusion_frequencies.keys():
        backup_drop_ratios[net] = drop_counters_per_backup_network[net] / backup_network_inclusion_frequencies[net]

    print(backup_drop_ratios)

    print(drop_counters_per_num_ues)

    df = pd.DataFrame(data=datapoints)
    df["auths_per_second"] = df["total_test_auth_count"] / df["total_test_duration_s"]
    df["auth_ms"] = df["auth_ns"] / float(10**6)
    df["registration_ms"] = df["registration_ns"] / float(10**6)
    df["session_ms"] = df["session_ns"] / float(10**6)

    return df

def extract_metadata_from_test_name(name_string: str) -> dict[str, str]:
    matches = backup_metadata_extraction_regex.fullmatch(name_string)
    if len(matches.groups()) != 6:
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
    }
    log.debug("Parsed test metadata: %s", res)

    return res

def extract_metadata_from_filename(name_string: str):
    match = filename_metadata_extraction_regex.fullmatch(name_string)
    return match.groups()[0]

def extract_ue_number_from_imsi(imsi: str) -> int:
    match = user_sim_number.match(imsi)
    if match is None or len(match.groups()) != 1:
        raise ValueError("Invalid imsi parsed")

    return int(match.groups()[0])


def make_plot(df: pd.DataFrame, chart_output_path: Path):
    chart_output_path.mkdir(parents=True, exist_ok=True)
    df = df.loc[df["serving_network"] == "Hestia-service"]
    df = df.loc[(df["home_network"] != "uwbts3-service") & (df["home_network"] != "uwbts2-service")]

    print(len(df), df.head())

    #alt.Chart(df).mark_line(opacity=0.5, interpolate='step-after').encode(
    alt.Chart(df).mark_boxplot().encode(
        x=alt.X(
            "auths_per_second:Q"
        ),
        y=alt.Y(
            "registration_ns:Q",
            title="ns_register",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "total_test_auth_count:N",
            scale=alt.Scale(scheme="tableau20"),
        ),
    ).properties(
        width=500,
    ).save(
        chart_output_path/"backup_auth_tmp.png",
        scale_factor=2,
    )

def generate_cdf_series(df, filter_column, filter_value, value_column):
    stats_frame = df
    stats_frame = stats_frame.loc[stats_frame[filter_column] == filter_value]
    stats_frame = stats_frame.groupby([value_column]).count()[["user_id"]].rename(columns = {"user_id": "sample_count"})
    stats_frame["pdf"] = stats_frame["sample_count"] / sum(stats_frame["sample_count"])
    stats_frame["cdf"] = stats_frame["pdf"].cumsum()
    stats_frame[filter_column] = filter_value
    stats_frame = stats_frame.reset_index()
    return stats_frame

def make_latency_cdf_small_multiple(number_ues, df: pd.DataFrame, chart_output_path: Path):
    # Filter to a particular load level and threshold:
    load_level = number_ues * 10
    df = df.loc[(df["total_test_auth_count"] == load_level) & (df["threshold"] == 4)]

    # Compute a cdf over observed latencies separately for each scenario
    plot_frame = None
    for scenario in df["scenario"].unique():
        if plot_frame is None:
            plot_frame = generate_cdf_series(df, "scenario", scenario, "registration_ms")
        else:
            plot_frame = pd.concat([plot_frame, generate_cdf_series(df, "scenario", scenario, "registration_ms")], ignore_index=True)

    plot_frame = plot_frame.reset_index()
    alt.Chart(plot_frame).mark_line(interpolate="step-after", clip=True).encode(
        x=alt.X('registration_ms:Q',
                scale=alt.Scale(type="linear", domain=[0, 1000]),
                title="Time to Complete Registration (ms)"
                ),
        y=alt.Y('cdf',
                title="CDF of Samples",
                scale=alt.Scale(type="linear", domain=[0.0, 1.0])
                ),
        color=alt.Color(
            "scenario:N",
            scale=alt.Scale(scheme="tableau10"),
        )
    ).properties(
        width=500,
        height=200,
    ).save(chart_output_path/f"backup_latency_vs_cloud_cdf_{number_ues}_ues.png", scale_factor=2.0)

def make_all_latency_cdfs(df: pd.DataFrame, chart_output_path: Path):
    for num_ues in [1, 5, 10, 20, 50]:
        make_latency_cdf_small_multiple(num_ues, df, chart_output_path)

if __name__ == "__main__":
    intermediate_path = Path("scratch")
    charts_path = intermediate_path/"renders"
    # Read in and build parquet from raw data df =
    df = normalize_json_to_dataframe(Path("data/ueransim/dauth/metric_set_2"))
    intermediate_path.mkdir(exist_ok=True)
    df.to_parquet(intermediate_path/"backup_network_via_dauth.parquet")

    df = pd.read_parquet(intermediate_path/"backup_network_via_dauth.parquet")
    print(df.head())
    make_plot(df, charts_path)
    make_all_latency_cdfs(df, charts_path)
