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
log.setLevel(logging.INFO)

# Regex is non-trivial to account for a typo in some of the early logs.
attach_extraction_regex = re.compile(r"Attach_time_nao?nos: ?([0-9]+)")

def canonicalize_all_logs(input_dir, intermediate_file):
    """ Processes all logs into a single dataframe for further analysis
    """
    # Read in and build parquet from raw data
    dataframes = [
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-open5gs-nova233-cbrs-20MHz-unloaded.log", -1, "open5gs", "unloaded"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-open5gs-nova233-cbrs-20MHz-loaded-iperf.log", -1, "open5gs", "loaded"),

        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-nova233-cbrs-20MHz-unloaded.log", -1, "dauth-home-online", "unloaded"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-nova233-cbrs-20MHz-unloaded-accidental-dupe.log", -1, "dauth-home-online", "unloaded"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-bt-6-nova233-cbrs-20MHz-unloaded.log", 6, "dauth-backup", "unloaded"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-bt-4-nova233-cbrs-20MHz-unloaded.log", 4, "dauth-backup", "unloaded"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-bt-2-nova233-cbrs-20MHz-unloaded.log", 2, "dauth-backup", "unloaded"),
    ]
    df = pd.concat(dataframes, axis=0)
    intermediate_file.parent.mkdir(exist_ok=True)
    df.to_parquet(intermediate_file)
    log.info("Successfully parsed %d rows from all input files", len(df))

def normalize_log_to_dataframe(filename, threshold, condition, load):
    """ Parses and processes a single log into a dataframe for further analysis
    """
    datapoints = []

    with open(filename) as f:
        for line in f:
            attach_time_matches = attach_extraction_regex.search(line)
            if not attach_time_matches:
                log.debug("Dropping line: %s", line)
                continue

            if len(attach_time_matches.groups()) != 1:
                log.warning("Multiple matches, data malformed on line: %s", line)
                continue

            log.debug("Appending line %s", line)
            attach_time_ns = int(attach_time_matches.groups()[0])
            datapoints.append({
                "attach_time_ms": float(attach_time_ns)/1_000_000.0,
                "threshold": threshold,
                "condition": condition,
                "load": load,
                })

    log.info("[%s] Successfully parsed %d rows", filename, len(datapoints))
    df = pd.DataFrame(data=datapoints)

    def label_maker(row):
        out = row["condition"] + "-" + row["load"]
        if row["threshold"] > 0:
            out = out + "-threshold[" +str(row["threshold"]) + "]"
        return out

    df["label"] = df.apply(label_maker, axis=1)

    return df

def make_boxplots(df: pd.DataFrame, chart_output_path: Path):
    chart_output_path.mkdir(parents=True, exist_ok=True)
    print(len(df), df.head())

    alt.Chart(df).mark_boxplot().encode(
        x=alt.X(
            "label:O"
        ),
        y=alt.Y(
            "attach_time_ms:Q",
            title="Attach Time (ms)",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "condition:N",
            scale=alt.Scale(scheme="tableau20"),
        ),
    ).properties(
        width=500,
    ).save(
        chart_output_path/"srsran-complete-boxplot.png",
        scale_factor=2,
    )

    df = df.loc[df["attach_time_ms"] < 1000.0]

    alt.Chart(df).mark_boxplot().encode(
        x=alt.X(
            "label:O"
        ),
        y=alt.Y(
            "attach_time_ms:Q",
            title="Attach Time (ms)",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "condition:N",
            scale=alt.Scale(scheme="tableau20"),
        ),
    ).properties(
        width=500,
    ).save(
        chart_output_path/"srsran-boxplot-drop-outliers.png",
        scale_factor=2,
    )

def generate_cdf_series(df, filter_column, filter_value, value_column):
    stats_frame = df.reset_index()
    stats_frame = stats_frame.loc[stats_frame[filter_column] == filter_value]
    stats_frame = stats_frame.groupby([value_column]).count()[["index"]].rename(columns = {"index": "sample_count"})
    stats_frame["pdf"] = stats_frame["sample_count"] / sum(stats_frame["sample_count"])
    stats_frame["cdf"] = stats_frame["pdf"].cumsum()
    stats_frame[filter_column] = filter_value
    stats_frame = stats_frame.reset_index()
    return stats_frame

def make_cdfs(df: pd.DataFrame, chart_output_path: Path):
    chart_output_path.mkdir(parents=True, exist_ok=True)

    # Compute a cdf over observed latencies separately for each scenario
    plot_frame = None
    for scenario in df["label"].unique():
        if plot_frame is None:
            plot_frame = generate_cdf_series(df, "label", scenario, "attach_time_ms")
        else:
            plot_frame = pd.concat([plot_frame, generate_cdf_series(df, "label", scenario, "attach_time_ms")], ignore_index=True)

    df = plot_frame.reset_index()

    alt.Chart(df).mark_line().encode(
        x=alt.X(
            "attach_time_ms:Q"
        ),
        y=alt.Y(
            "cdf:Q",
            title="Attach Time (ms)",
            axis=alt.Axis(labels=True),
            # scale=alt.Scale(
            #     type="symlog"
            # ),
        ),
        color=alt.Color(
            "label:N",
            scale=alt.Scale(scheme="tableau20"),
        ),
    ).properties(
        width=500,
    ).save(
        chart_output_path/"srsran-complete-cdfs.png",
        scale_factor=2,
    )

if __name__ == "__main__":
    raw_logs_path = Path("data/srsran/raw")

    intermediate_file = Path("scratch/srsran_tests.parquet")

    # Read and process all raw input logs
    canonicalize_all_logs(raw_logs_path, intermediate_file)

    # Make charts from input data
    chart_renders_dir = Path("scratch/renders")
    df = pd.read_parquet(intermediate_file)
    print(df.head())
    # make_boxplots(df, chart_renders_dir)
    make_cdfs(df, chart_renders_dir)
