import os
from logging import INFO

from logger import TestingLogger
from tests.common import TestingConfig, TestingState, TestingException
from tests.config import ServiceConfig


class LocalAuthTests:
    def __init__(self, state: TestingState) -> None:
        self.state = state
        self.gnb_config_path = "./configs/ueransim/gnb-1.yaml"
        self.ue_config_path = "./configs/ueransim/ue.yaml"
        
    def _configure_tests(self, num_users: int):
        """
        Handles all common configuration for local auth tests.
        """
        TestingLogger.logger.info("Configuring for {} UE(s) in local auth".format(num_users))
    
        # Configure all unused state to use default empty config.
        # For local authentication, only one service is used.
        self.state.service2.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service2.yaml")))
        self.state.service3.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service3.yaml")))
        self.state.service4.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service4.yaml")))

        service_config = ServiceConfig(
            os.path.join(self.state.config_dir, "service1.yaml"))

        sqn_slice_max = {0: 32}
        backup_network_ids = dict()

        if num_users < 1:
            raise TestingException("Number of users is less than 1")
        elif num_users > 9999999999:  # can represent 910540000000001-910549999999999
            raise TestingException("Too many users to represent")
        else:
            for i in range(num_users):
                imsi = "imsi-91054{}".format(str(i+1).zfill(10))
                service_config.add_user(imsi, sqn_slice_max, backup_network_ids)
        
        self.state.service1.change_config(service_config)

    def _test_single_connection(self):
        """
        Tests that a UE can connect locally to its home network.
        """
        TestingLogger.logger.info("Running test_single_connection")

        # Configure the tests and reset the state to apply changes
        self._configure_tests(1)
        self.state.reset()
        
        # simply connect and see if successful
        gnb = self.state.start_and_check_gnb(self.gnb_config_path)
        ue = self.state.start_and_check_ue(self.ue_config_path, "imsi-910540000000001", 1)
        
    def _test_multiple_connections(self):
        """
        Tests that a UE can connect locally to its home network.
        """
        TestingLogger.logger.info("Running test_multiple_connection")

        # Configure the tests and reset the state to apply changes
        self._configure_tests(10)
        self.state.reset()
        
        # simply connect and see if successful
        gnb = self.state.start_and_check_gnb(self.gnb_config_path)
        ue = self.state.start_and_check_ue(self.ue_config_path, "imsi-910540000000001", 10)

    def run_tests(self):
        """
        Run all tests in for local authentication.
        Since VM setup is required, tests should not be run outside this function.
        """
        
        TestingLogger.logger.setLevel(INFO)
        TestingLogger.logger.info("Starting local auth tests")
        
        try: 
            # Run all tests
            self._test_single_connection()
            self._test_multiple_connections()
        except TestingException:
            TestingLogger.logger.error("Last test failed")
        else:
            TestingLogger.logger.info("All tests passed")
        
        TestingLogger.logger.info("Tests completed")
        self.state.reset()
    
if __name__ == "__main__":
    # Build default test setup and run tests
    TestingLogger.logger.info("Building state and connections...")
    state = TestingState(TestingConfig('./', './testing/configs'))
    LocalAuthTests(state).run_tests()
