import sys
sys.path.append("../protos")
sys.path.append("..")

from tests import ping_tests, log_tests

# Put any tests to run in this file

tester = ping_tests.PingTester(host="192.168.0.9", port=13172)
tester.ping_self(num_pings=200, ping_delay=0.001)
tester.ping_multiple(["192.168.0.7:13172"],num_pings=100, ping_delay=0.001)
log_tests.log_test()

