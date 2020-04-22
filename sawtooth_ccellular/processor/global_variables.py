import getpass
import random
import string
from network import NetworkManager, services


def random_string(string_length=10):
    """Generate a random string of fixed length """
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for i in range(string_length))


# This class is used to initialize and hold all globally accessable variables in the program
class GlobalVariables:
    DISTRIBUTION_NAME = 'sawtooth-ccellular'
    DEFAULT_URL = 'tcp://172.17.0.1:4004'

    CLIENT_DEFAULT_URL = 'http://172.17.0.1:8008'
    CLIENT_DEFAULT_KEY_PATH = '~/.sawtooth/keys/' + getpass.getuser() + '.priv'

    FAMILY_NAME = 'ccellular'
    FAMILY_VERSION = '1.0'

    DB_CONNECTION_HOST = 'localhost'
    DB_CONNECTION_PORT = 27017
    DB_COLLECTION_NAME = 'subscribers'
    DB_DATABASE_NAME = 'open5gs'

    # Network Manager
    NWM_PORT = 13127
    NWM_PRIORITIES = [0, 1, 2]
    NWM_LIMIT_PRIORITIES = True
    NWM_LOGFILE_DIR = './output/client_logs'
    NWM = None

    # Services
    LOGGING_SERVER_HOST = "localhost"
    LOGGING_SERVER_PORT = 13173
    LOGGER = None

    PROCESSOR_NAME = random_string()

    # Initialize all global variables
    @staticmethod
    def init_globals(global_kv):

        # Set database name
        if "dbname" in global_kv:
            GlobalVariables.DB_DATABASE_NAME = global_kv["dbname"]

        print("Assigning PROCESSOR_NAME as {}".format(GlobalVariables.PROCESSOR_NAME))

    @staticmethod
    def init_network_manager(use_defualt_services=True):
        # Build a network manager
        GlobalVariables.NWM = NetworkManager(port=GlobalVariables.NWM_PORT,
                                             known_priorities=GlobalVariables.NWM_PRIORITIES,
                                             limit_to_known_priorities=GlobalVariables.NWM_LIMIT_PRIORITIES,
                                             logfile_dir=GlobalVariables.NWM_LOGFILE_DIR
                                             )

        # Add default services
        if use_defualt_services:
            service_list = GlobalVariables.build_default_services()

            for service in service_list:
                GlobalVariables.NWM.add_service(service)

    @staticmethod
    def build_default_services():
        service_list = []
        service_list.append(services.DebugPing())
        GlobalVariables.LOGGER = services.LoggingClient(host=GlobalVariables.LOGGING_SERVER_HOST, port=GlobalVariables.LOGGING_SERVER_PORT)
        service_list.append(GlobalVariables.LOGGER)
        return service_list

        
