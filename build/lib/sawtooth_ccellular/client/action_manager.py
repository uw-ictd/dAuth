import getpass
import os

from sawtooth_ccellular.client.ccellular_client import CCellularClient
from sawtooth_ccellular.client.constants import DEFAULT_URL


def _get_client(args, read_key_file=True):
    return CCellularClient(url=DEFAULT_URL, keyfile=_get_keyfile(args) if read_key_file else None)


def _get_keyfile(args):
    real_user = getpass.getuser()
    home = os.path.expanduser("~")
    key_dir = os.path.join(home, ".sawtooth", "keys")
    return '{}/{}.priv'.format(key_dir, real_user)


def do_set(args):
    imsi, auth_vector = args.name, args.value
    client = _get_client(args)
    response = client.set(imsi, auth_vector)
    print(imsi, response)


def do_get(args):
    imsi = args.name
    print('Received instruction to get {}'.format(imsi))
    client = _get_client(args)
    response = client.get(imsi)
    print(response)
