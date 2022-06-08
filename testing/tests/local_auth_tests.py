import os
from logging import INFO

from logger import TestingLogger
from tests.common import TestingConfig, TestingState, TestingException
from tests.config import ServiceConfig, UEConfig


class LocalAuthTests:
    def __init__(self, state: TestingState) -> None:
        self.state = state
        self.gnb_config_path = "./configs/ueransim/gnb-1.yaml"
        self.ue_config_path = "./testing-ue.yaml"
        
    def _configure_tests(self):
        """
        Handles all common configuration for local auth tests.
        """
        TestingLogger.logger.info("Configuring vms for local auth testing")
    
        # Configure all unused state to use default empty config.
        # For local authentication, only one service is used.
        self.state.service2.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service2.yaml")))
        self.state.service3.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service3.yaml")))
        self.state.service4.change_config(
            ServiceConfig(os.path.join(self.state.config_dir, "service4.yaml")))
        
        # user information
        imsi = "imsi-901700000000001"
        sqn_slice_max = {0: 32}
        backup_network_ids = dict()
        
        # configure the used service
        service_config = ServiceConfig(
            os.path.join(self.state.config_dir, "service1.yaml"))
        service_config.add_user(imsi, sqn_slice_max, backup_network_ids)
        self.state.service1.change_config(service_config)
        
        # configure the ue
        ue_config = UEConfig(
            os.path.join(self.state.config_dir, "ue.yaml"))
        ue_config.set_imsi(imsi)
        self.state.ueransim.add_ue_config(ue_config, self.ue_config_path)
            
    def _test_single_connection(self):
        """
        Tests that a UE can connect locally to its home network.
        """
        TestingLogger.logger.info("Running test_single_connection")
        self.state.reset()  # always reset before starting tests
        
        gnb = self.state.start_and_check_gnb(self.gnb_config_path)
        ue = self.state.start_and_check_ue(self.ue_config_path)

    def run_tests(self):
        """
        Run all tests in for local authentication.
        Since VM setup is required, tests should not be run outside this function.
        """
        
        TestingLogger.logger.setLevel(INFO)
        TestingLogger.logger.info("Starting local auth tests")

        self._configure_tests()
        
        try: 
            # Run all tests
            self._test_single_connection()
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
