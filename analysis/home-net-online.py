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

home_metadata_extraction_regex = re.compile(r"^home_auth:<H,S>\((.+),(.+)\):<n,i,t>\(([0-9]+),([0-9]+),([0-9]+)\)$")
user_sim_number = re.compile(r"^90170([0-9]+)$")

def normalize_json_to_dataframe(result_directory_path: Path):
    result_filenames = result_directory_path.glob("*")
    result_filenames = list(result_filenames)
    result_filenames.sort()

    datapoints = []
    for filename in result_filenames:
        with open(filename) as f:
            lines = []
            for line in f:
                lines.append(line)

            for i, line in enumerate(lines):
                parsed_json = json.loads(line)

                if i%2 == 0:
                    # The line is a high-level result line
                    test_parameters = extract_metadata_from_test_name(parsed_json["test_name"])
                    test_parameters["total_test_duration_s"] = float(parsed_json["test_duration"])
                    test_parameters["total_test_auth_count"] = int(parsed_json["total_auths"])
                    print(test_parameters)

                    # Attempt to parse timing data from each provided UE.
                    for key in parsed_json.keys():
                        try:
                            user_number = extract_ue_number_from_imsi(key)
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

                    print(datapoints[0])
                else:
                    # The line is a tokio timing line
                    log.debug("Not using tokio timing for now")

    df = pd.DataFrame(data=datapoints)
    df["auths_per_second"] = df["total_test_auth_count"] / df["total_test_duration_s"]

    return df

def extract_metadata_from_test_name(name_string: str) -> dict[str, str]:
    matches = home_metadata_extraction_regex.fullmatch(name_string)
    if len(matches.groups()) != 5:
        raise ValueError("Invalid test name parsed")

    result_groups = matches.groups()
    res = {
        "home_network": result_groups[0],
        "serving_network": result_groups[1]
    }
    log.debug("Parsed test metadata: %s", res)

    return res

def extract_ue_number_from_imsi(imsi: str) -> int:
    match = user_sim_number.fullmatch(imsi)
    if match is None or len(match.groups()) != 1:
        raise ValueError("Invalid imsi parsed")

    return int(match.groups()[0])


def make_plot(df: pd.DataFrame, chart_output_path: Path):
    chart_output_path.mkdir(parents=True, exist_ok=True)
    df = df.loc[df["serving_network"] == "Cobble-service"]
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
            "home_network:N",
            scale=alt.Scale(scheme="tableau20"),
        ),
    ).properties(
        width=500,
    ).save(
        chart_output_path/"home_auth_tmp.png",
        scale_factor=2,
    )


if __name__ == "__main__":
    intermediate_path = Path("scratch")
    charts_path = intermediate_path/"renders"
    # Read in and build parquet from raw data
    # df = normalize_json_to_dataframe(Path("data/ueransim/dauth/metric_set_1"))
    # output_path.mkdir(exist_ok=True)
    # df.to_parquet(output_path/"home_network_via_dauth.parquet")

    df = pd.read_parquet(intermediate_path/"home_network_via_dauth.parquet")
    print(df.head())
    make_plot(df, charts_path)
