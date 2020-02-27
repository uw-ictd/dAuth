import getpass
import random
import string


def random_string(string_length=10):
    """Generate a random string of fixed length """
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for i in range(string_length))

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

MESSAGING_HOST = 'localhost'
MESSAGING_PORT = 13127
MESSAGING_LOGFILE = './output/client_logs'
MESSAGING_STREAM_MAX = 10000

PROCESSOR_NAME = random_string()

print("Assigning PROCESSOR_NAME as {}".format(PROCESSOR_NAME))
