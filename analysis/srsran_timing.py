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
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-open5gs-nova233-cbrs-20MHz-unloaded.log", -1, "open5gs"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-nova233-cbrs-20MHz-unloaded.log", 8, "dauth"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-nova233-cbrs-20MHz-unloaded-accidental-dupe.log", 8, "dauth"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-bt-6-nova233-cbrs-20MHz-unloaded.log", 6, "dauth"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-bt-4-nova233-cbrs-20MHz-unloaded.log", 4, "dauth"),
        normalize_log_to_dataframe(input_dir/"srslte-ue-attach-loop-dauth-s-h-cobble-dauth-hestia-bt-2-nova233-cbrs-20MHz-unloaded.log", 2, "dauth")
    ]
    df = pd.concat(dataframes, axis=0)
    intermediate_file.parent.mkdir(exist_ok=True)
    df.to_parquet(intermediate_file)
    log.info("Successfully parsed %d rows from all input files", len(df))

def normalize_log_to_dataframe(filename, threshold, condition):
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
                })

    log.info("[%s] Successfully parsed %d rows", filename, len(datapoints))
    return pd.DataFrame(data=datapoints)


if __name__ == "__main__":
    raw_logs_path = Path("data/srsran/raw")

    intermediate_file = Path("scratch/srsran_tests.parquet")

    # Read and process all raw input logs
    canonicalize_all_logs(raw_logs_path, intermediate_file)

    # Make charts from input data
    chart_renders_dir = Path("scratch/renders")
    df = pd.read_parquet(intermediate_file)
    print(df.head())
    # make_plot(df, charts_path)
