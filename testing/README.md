## dAuth integration testing
Functional tests are under `perf/`.
Scripts to manage dAuth and directory instances are under `scripts/`.


## Perf testing
Performance tests under `perf/` can be used with either VMs, real machines, or both.

### Running a simple test
To run a single test, use the scripts under `sample_scripts/`.

### Advanced perf tests
There are numerous tests under `perf/dauth_testing` that are meant to run a large number
of performance tests and record the results. The scripts autogenerate a number of tests from
the base configs and the an increasing number of UEs. NOTE: These configs are based on a real
physical setup, and would require changes to be used on another setup.


## Scripts
Scripts under `scripts/` allow control and monitoring of dauth and directory nodes when doing any kinds of testing.
Perf tests will automatically clean up, but if running custom tests these scripts can be used to reset node.
Likewise, logs can be streamed remotely during any test.
