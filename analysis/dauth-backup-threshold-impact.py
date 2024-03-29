from collections import defaultdict
from pathlib import Path
import json
import logging
import re

import altair as alt
import pandas as pd

import constants
import helpers

# Module specific format options
pd.set_option('display.max_columns', None)
pd.set_option('display.max_colwidth', None)
pd.set_option('display.width', None)
pd.set_option('display.max_rows', 80)

logging.basicConfig()

log = logging.getLogger(__name__)
log.setLevel(logging.DEBUG)

user_sim_number = re.compile(r"^90170([0-9]+)$")

def p50(x):
    return x.quantile(0.5)

def p90(x):
    return x.quantile(0.9)

def p95(x):
    return x.quantile(0.95)

def p99(x):
    return x.quantile(0.99)

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
        filename_parameters = helpers.extract_metadata_from_backup_filename(filename.name)
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
                    test_parameters = helpers.extract_metadata_from_backup_test_name(parsed_json["test_name"])
                except KeyError:
                    # The line is a tokio timing line we're not using for now
                    pass
                    continue

                test_parameters = test_parameters | filename_parameters
                test_parameters["total_test_duration_s"] = float(parsed_json["test_duration"])
                test_parameters["total_test_auth_count"] = int(parsed_json["total_auths"])

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
                    drop_count += 1
                    drop_counters_per_num_ues[test_parameters["total_test_auth_count"]] += 1
                    for net in test_parameters["backup_networks"]:
                        drop_counters_per_backup_network[net] += 1

                for net in test_parameters["backup_networks"]:
                    backup_network_inclusion_frequencies[net] += 1

    backup_drop_ratios = dict()
    for net in backup_network_inclusion_frequencies.keys():
        backup_drop_ratios[net] = drop_counters_per_backup_network[net] / backup_network_inclusion_frequencies[net]

    df = pd.DataFrame(data=datapoints)
    df["auths_per_second"] = df["total_test_auth_count"] / df["total_test_duration_s"]
    df["auth_ms"] = df["auth_ns"] / float(10**6)
    df["registration_ms"] = df["registration_ns"] / float(10**6)

    return df

def extract_metadata_from_filename(name_string: str):
    match = filename_metadata_extraction_regex.fullmatch(name_string)
    return match.groups()[0]

def extract_ue_number_from_imsi(imsi: str) -> int:
    match = user_sim_number.match(imsi)
    if match is None or len(match.groups()) != 1:
        raise ValueError("Invalid imsi parsed")

    return int(match.groups()[0])


def make_scenario_plot(df: pd.DataFrame, chart_output_path: Path, scenario:str):
    chart_output_path.mkdir(parents=True, exist_ok=True)
    df = df.loc[(df["scenario"] == scenario) & (df["backup_count"] == 8)]
    #df = df.loc[(df["ue_count"] < 30) & (df["backup_count"] == 8)]

    stats = df.groupby(["ue_count", "threshold", "test_seed"]).agg({"registration_ms": [p50, p90, p95, p99]})

    # Flatten the dataframe for altair
    stats = stats.reset_index()
    stats.columns = stats.columns = ['_'.join(col).strip().strip("_") for col in stats.columns.values]

    alt.Chart(stats).mark_point().encode(
        x=alt.X(
            "ue_count:Q"
        ),
        y=alt.Y(
            "registration_ms_p99:Q",
            title="ns_register",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "threshold:O",
            scale=alt.Scale(scheme="category10"),
        ),
        # shape=alt.Shape(
        #     "quantile:N"
        # ),
        strokeDash=alt.StrokeDash(
            "threshold:O"
        ),
        detail=alt.Detail(
            "test_seed:N"
        )
    ).properties(
        width=500,
    ).save(
        chart_output_path/f"threshold_impact_scenario_{scenario}_q99_samples.png",
        scale_factor=2,
    )

    alt.Chart(stats).mark_boxplot().encode(
        x=alt.X(
            "threshold:O"
        ),
        y=alt.Y(
            "registration_ms_p99:Q",
            title="ns_register",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "threshold:O",
            scale=alt.Scale(scheme="category10"),
        ),
    ).properties(
        width=500,
    ).facet(
        column="ue_count:N"
    ).save(
        chart_output_path/f"threshold_impact_scenario_{scenario}_small_multiple.png",
        scale_factor=2,
    )

    agg_stats = df.groupby(["ue_count", "threshold"]).agg({"registration_ms": [p50, p90, p95, p99]})

    # Flatten the dataframe for altair
    agg_stats = agg_stats.reset_index()
    agg_stats.columns = agg_stats.columns = ['_'.join(col).strip().strip("_") for col in agg_stats.columns.values]

    agg_stats = agg_stats.melt(
        id_vars=["ue_count", "threshold"],
        value_vars=["registration_ms_p50", "registration_ms_p90", "registration_ms_p95", "registration_ms_p99"],
        var_name="quantile",
        value_name="registration_ms",
    )

    # alt.Chart(agg_stats).mark_line(opacity=0.5, interpolate='step-after').encode(
    alt.Chart(agg_stats).mark_line().encode(
        x=alt.X(
            "ue_count:Q",
            title="Authentications Per Minute"
        ),
        y=alt.Y(
            "registration_ms:Q",
            title="Attach Time (ms)",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "threshold:O",
            scale=alt.Scale(scheme="category10"),
            title="Threshold",
        ),
        shape=alt.Shape(
            "quantile:N",
            title="Quantile"
        ),
        detail=alt.Detail(
            "quantile:N"
        )
    ).properties(
        width=500,
    ).save(
        chart_output_path/f"threshold_impact_scenario_{scenario}.png",
        scale_factor=2,
    )

def make_combined_plot(df: pd.DataFrame, chart_output_path: Path):
    chart_output_path.mkdir(parents=True, exist_ok=True)
    df = df.loc[(df["backup_count"] == 8)]

    agg_stats = df.groupby(["ue_count", "threshold"]).agg({"registration_ms": [p50, p90, p95, p99]})

    # Flatten the dataframe for altair
    agg_stats = agg_stats.reset_index()
    agg_stats.columns = agg_stats.columns = ['_'.join(col).strip().strip("_") for col in agg_stats.columns.values]

    agg_stats = agg_stats.melt(
        id_vars=["ue_count", "threshold"],
        value_vars=["registration_ms_p50", "registration_ms_p90", "registration_ms_p95", "registration_ms_p99"],
        var_name="quantile",
        value_name="registration_ms",
    )

    agg_stats = agg_stats.replace({"registration_ms_p50":"p50", "registration_ms_p90":"p90", "registration_ms_p95":"p95", "registration_ms_p99":"p99"})

    # alt.Chart(agg_stats).mark_line(opacity=0.5, interpolate='step-after').encode(
    alt.Chart(agg_stats).mark_line(opacity=0.8).encode(
        x=alt.X(
            "ue_count:Q",
            title="Authentications Per Minute"
        ),
        y=alt.Y(
            "registration_ms:Q",
            title="Attach Time (ms)",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "threshold:O",
            scale=alt.Scale(scheme="category10"),
            title="Threshold",
        ),
        strokeDash=alt.StrokeDash(
            "threshold:O",
            title="Threshold"
        ),
        shape=alt.Shape(
            "quantile:N",
            title="Quantile"
        ),
        detail=alt.Detail(
            "quantile:N"
        )
    ).properties(
        width=500,
    ).save(
        chart_output_path/"threshold_impact_combined.png",
        scale_factor=2,
    )

def make_all_scenario_plots(df, chart_output_path):
    scenarios = []
    for string in ["1", "2", "3", "4"]:
        scenarios.append(constants.label_from_scenario(string))
    for scenario in scenarios:
        make_scenario_plot(df, chart_output_path, scenario)

if __name__ == "__main__":
    intermediate_path = Path("scratch")
    charts_path = intermediate_path/"renders"
    # Read in and build parquet from raw data df =
    df = normalize_json_to_dataframe(Path("data/ueransim/dauth/metric_set_2"))
    df["ue_count"] = df["ue_count"] * 2

    make_all_scenario_plots(df, charts_path)
    make_combined_plot(df, charts_path)
