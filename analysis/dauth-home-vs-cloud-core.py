from collections import defaultdict
from pathlib import Path
import json
import logging
import re

import altair as alt
import pandas as pd

import constants

# Module specific format options
pd.set_option('display.max_columns', None)
pd.set_option('display.max_colwidth', None)
pd.set_option('display.width', None)
pd.set_option('display.max_rows', 40)

logging.basicConfig()

log = logging.getLogger(__name__)
log.setLevel(logging.INFO)

home_metadata_extraction_regex = re.compile(r"^home_auth:<H,S>\((.+),(.+)\):<n,i,t>\(([0-9]+),([0-9]+),([0-9]+)\)$")
backup_metadata_extraction_regex = re.compile(r"^backup_auth:<H,S,B,T>\(([A-Z,a-z,\-]+),([A-Z,a-z,\-]+),(\[.+\])\):<n,i,t>\(([0-9]+),([0-9]+),([0-9]+)\)$")
home_filename_metadata_extraction_regex = re.compile(r"^([0-9]+)-.*\.out$")
user_sim_number = re.compile(r"^90170([0-9]+)$")

def normalize_json_to_dataframe(result_directory_path: Path):
    result_filenames = result_directory_path.glob("*")
    result_filenames = list(result_filenames)
    result_filenames.sort()

    datapoints = []
    for filename in result_filenames:
        match = home_filename_metadata_extraction_regex.fullmatch(filename.name)
        scenario = match.groups()[0]
        with open(filename) as f:
            lines = []
            for line in f:
                lines.append(line)

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
                test_parameters["scenario"] = constants.label_from_scenario(scenario)

                # name_appearance_count[parsed_json["test_name"]] += 1

                # if name_appearance_count[parsed_json["test_name"]] == 1:
                #     test_parameters["threshold"] = 2
                # elif name_appearance_count[parsed_json["test_name"]] == 2:
                #     test_parameters["threshold"] = 4
                # elif name_appearance_count[parsed_json["test_name"]] == 3:
                #     test_parameters["threshold"] = 8
                # else:
                #     raise ValueError("Unknown threshold encountered, results possibly corrupt")

                # Attempt to parse timing data from each provided UE.
                found_ues = 0

                for i, iteration in enumerate(zip(parsed_json["nanoseconds_since_auth"], parsed_json["nanoseconds_since_registration"])):
                    datapoint = {
                        "user_id": i,
                        "auth_ns": iteration[0],
                        "registration_ns": iteration[1],
                        }
                    datapoint = datapoint | test_parameters
                    datapoints.append(datapoint)
                    found_ues += 1

                if found_ues == 0:
                    log.warning("Test failed to execute and has no returned results, dropping from analysis: %s", parsed_json["test_name"])

            # for i, line in enumerate(lines):
            #     parsed_json = json.loads(line)

            #     if i%2 == 0:
            #         # The line is a high-level result line
            #         test_parameters = extract_metadata_from_test_name(parsed_json["test_name"])
            #         test_parameters["total_test_duration_s"] = float(parsed_json["test_duration"])
            #         test_parameters["total_test_auth_count"] = int(parsed_json["total_auths"])
            #         test_parameters["scenario"]= constants.label_from_scenario(scenario)
            #         print(test_parameters)

            #         # Attempt to parse timing data from each provided UE.
            #         for key in parsed_json.keys():
            #             try:
            #                 user_number = extract_ue_number_from_imsi(key)
            #             except ValueError:
            #                 # Continue looking at other keys if this key is not a valid imsi
            #                 continue

            #             ue_results = parsed_json[key]["results"]
            #             for iteration in zip(ue_results["nanoseconds_since_auth"], ue_results["nanoseconds_since_registration"], ue_results["nanoseconds_to_establish_session"]):
            #                 datapoint = {
            #                     "user_id": user_number,
            #                     "auth_ns": iteration[0],
            #                     "registration_ns": iteration[1],
            #                     "session_ns": iteration[2],
            #                     }
            #                 datapoint = datapoint | test_parameters
            #                 datapoints.append(datapoint)

            #     else:
            #         # The line is a tokio timing line
            #         log.debug("Not using tokio timing for now")

    df = pd.DataFrame(data=datapoints)
    df["auths_per_second"] = df["total_test_auth_count"] / df["total_test_duration_s"]
    df["auth_ms"] = df["auth_ns"] / float(10**6)
    df["registration_ms"] = df["registration_ns"] / float(10**6)

    return df

def extract_metadata_from_test_name(name_string: str) -> dict[str, str]:
    matches = home_metadata_extraction_regex.fullmatch(name_string)
    if len(matches.groups()) != 5:
        raise ValueError("Invalid test name parsed")

    result_groups = matches.groups()
    res = {
        "home_network": result_groups[0],
        "serving_network": result_groups[1],
        "ue_count": int(result_groups[2]),
    }
    log.debug("Parsed test metadata: %s", res)

    return res

def extract_metadata_from_filename(name_string: str):
    match = home_filename_metadata_extraction_regex.fullmatch(name_string)
    return match.groups()[0]

def extract_ue_number_from_imsi(imsi: str) -> int:
    match = user_sim_number.match(imsi)
    if match is None or len(match.groups()) != 1:
        raise ValueError("Invalid imsi parsed")

    return int(match.groups()[0])


cloud_metadata_extraction_regex = re.compile(r"^local_auth:<H>\((.+)\):<n,i,t>\(([0-9]+),([0-9]+),([0-9]+)\)$")
cloud_filename_extraction_regex = re.compile(r"^([0-9]+)-(.*).out$")

def normalize_cloud_json_to_dataframe(result_directory_path: Path):
    result_filenames = result_directory_path.glob("*.out")
    result_filenames = list(result_filenames)
    result_filenames.sort()

    datapoints = []
    for filename in result_filenames:
        match = cloud_filename_extraction_regex.fullmatch(filename.name)
        scenario = match.groups()[0]
        with open(filename) as f:
            lines = []
            for line in f:
                lines.append(line)

            for i, line in enumerate(lines):
                parsed_json = json.loads(line)

                # The line is a high-level result line
                test_parameters = extract_metadata_from_cloud_test_name(parsed_json["test_name"])
                test_parameters["total_test_duration_s"] = float(parsed_json["test_duration"])
                test_parameters["total_test_auth_count"] = int(parsed_json["total_auths"])
                test_parameters["scenario"]= constants.label_from_scenario(scenario)

                for i, iteration in enumerate(zip(parsed_json["nanoseconds_since_auth"], parsed_json["nanoseconds_since_registration"])):
                    datapoint = {
                        "user_id": i,
                        "auth_ns": iteration[0],
                        "registration_ns": iteration[1],
                        }
                    datapoint = datapoint | test_parameters
                    datapoints.append(datapoint)

    df = pd.DataFrame(data=datapoints)
    df["auths_per_second"] = df["total_test_auth_count"] / df["total_test_duration_s"]
    df["auth_ms"] = df["auth_ns"] / float(10**6)
    df["registration_ms"] = df["registration_ns"] / float(10**6)

    return df

def extract_metadata_from_cloud_test_name(name_string: str) -> dict[str, str]:
    matches = cloud_metadata_extraction_regex.fullmatch(name_string)
    if len(matches.groups()) != 4:
        raise ValueError("Invalid test name parsed")

    result_groups = matches.groups()
    res = {
        "home_network": result_groups[0],
        "ue_count": int(result_groups[1]),
    }
    log.debug("Parsed test metadata: %s", res)

    return res


def generate_cdf_series(df, filter_column, filter_value, value_column):
    stats_frame = df
    stats_frame = stats_frame.loc[stats_frame[filter_column] == filter_value]
    stats_frame = stats_frame.groupby([value_column]).count()[["user_id"]].rename(columns = {"user_id": "sample_count"})
    stats_frame["pdf"] = stats_frame["sample_count"] / sum(stats_frame["sample_count"])
    stats_frame["cdf"] = stats_frame["pdf"].cumsum()
    stats_frame[filter_column] = filter_value
    stats_frame = stats_frame.reset_index()
    return stats_frame

def make_latency_cdf_small_multiple(number_ues, df: pd.DataFrame, cloud_df: pd.DataFrame, chart_output_path: Path):
    # Filter to a particular load level and threshold:
    print(df["ue_count"].unique())
    df = df.loc[(df["ue_count"] == number_ues) & (df["home_network"] == "AWS-service")]

    # Compute a cdf over observed latencies separately for each scenario
    plot_frame = None
    for scenario in df["scenario"].unique():
        if plot_frame is None:
            plot_frame = generate_cdf_series(df, "scenario", scenario, "registration_ms")
        else:
            plot_frame = pd.concat([plot_frame, generate_cdf_series(df, "scenario", scenario, "registration_ms")], ignore_index=True)

    plot_frame = plot_frame.reset_index()
    plot_frame["system"] = "dauth"

    final_aggregated_frame = plot_frame

    # Generate the same CDFs from the plain open5gs dataset
    df = cloud_df.loc[cloud_df["ue_count"] == int(number_ues)]

    plot_frame = None
    for scenario in df["scenario"].unique():
        if plot_frame is None:
            plot_frame = generate_cdf_series(df, "scenario", scenario, "registration_ms")
        else:
            plot_frame = pd.concat([plot_frame, generate_cdf_series(df, "scenario", scenario, "registration_ms")], ignore_index=True)

    if plot_frame is None:
        log.warning("Skipping mismatched test size %d", number_ues)
        return
    plot_frame = plot_frame.reset_index()
    plot_frame["system"] = "open5gs"

    final_aggregated_frame = pd.concat([final_aggregated_frame, plot_frame], ignore_index=True)

    final_aggregated_frame = final_aggregated_frame.sort_values(by="cdf", kind="stable")
    final_aggregated_frame = final_aggregated_frame.sort_values(by="scenario", kind="stable")
    final_aggregated_frame = final_aggregated_frame.sort_values(by="system", kind="stable")
    # final_aggregated_frame = final_aggregated_frame.loc[final_aggregated_frame["system"] == "open5gs"]
    alt.Chart(final_aggregated_frame).mark_line(interpolate="step-after", clip=True, opacity=1.0).encode(
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
            legend=alt.Legend(
                orient="bottom-right",
                fillColor="white",
                labelLimit=500,
                padding=5,
                strokeColor="black",
            ),
        ),
        strokeDash=alt.StrokeDash(
            "system:N",
            legend=alt.Legend(
                orient="top-right",
                fillColor="white",
                labelLimit=500,
                padding=5,
                strokeColor="black",
            ),
        ),
        detail="system:N"
    ).properties(
        width=500,
        height=200,
    ).save(chart_output_path/f"home_latency_vs_cloud_cdf_{number_ues}_ues.png", scale_factor=2.0)

def make_all_latency_cdfs(df: pd.DataFrame, cloud_df: pd.DataFrame, chart_output_path: Path):
    # for num_ues in [10, 20, 50, 75, 100, 200, 300, 400, 500]:
    for num_ues in [10, 20, 50, 100, 500]:
        make_latency_cdf_small_multiple(num_ues, df, cloud_df, chart_output_path)

if __name__ == "__main__":
    intermediate_path = Path("scratch")
    charts_path = intermediate_path/"renders"
    # Read in and build parquet from raw data df =
    df = normalize_json_to_dataframe(Path("data/ueransim/dauth/metric_set_1"))

    cloud_data = normalize_cloud_json_to_dataframe(Path("data/ueransim/open5gs-edge-core"))
    make_all_latency_cdfs(df, cloud_data, charts_path)
