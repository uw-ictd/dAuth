import logging
import hashlib
import cbor

from sawtooth_sdk.processor.handler import TransactionHandler
from sawtooth_sdk.processor.exceptions import InvalidTransaction
from sawtooth_sdk.processor.exceptions import InternalError

from sawtooth_ccellular.processor.constants import FAMILY_NAME, FAMILY_VERSION
from sawtooth_ccellular.processor.database import DatabaseManager
from sawtooth_ccellular.processor.utils import deserialize_proto_database_instructions


def _sha512(data):
    return hashlib.sha512(data).hexdigest()


def get_prefix():
    return _sha512(FAMILY_NAME.encode('utf-8'))[0:6]


def make_ccellular_address(imsi):
    return get_prefix() + _sha512(imsi.encode('utf-8'))[64:]


CCELLULAR_ADDRESS_PREFIX = get_prefix()


class CCellularTransactionHandler(TransactionHandler):
    database = None

    def __init__(self):
        db = DatabaseManager()
        self.database = db

    @property
    def family_name(self):
        return FAMILY_NAME

    @property
    def family_versions(self):
        return [FAMILY_VERSION]

    @property
    def namespaces(self):
        return [CCELLULAR_ADDRESS_PREFIX]

    def apply(self, transaction, context):
        action, imsi, value = _unpack_transaction(transaction)
        state = _get_state_data(imsi, context)
        updated_state = _do_ccellular(action, imsi, value, state, self.database)
        _set_state_data(imsi, updated_state, context)


def _decode_transaction(transaction):
    try:
        content = cbor.loads(transaction.payload)
    except:
        raise InvalidTransaction('Invalid Transaction Payload Serialization Format')
    action = content['verb']
    imsi = content['imsi']
    value = content['value']
    return action, imsi, value


def _unpack_transaction(transaction):
    action, name, value = _decode_transaction(transaction)
    # TODO: Perform necessary validations on this data here
    return action, name, value


def _get_state_data(imsi, context):
    address = make_ccellular_address(imsi)
    state_entries = context.get_state([address])
    try:
        return cbor.loads(state_entries[0].data)
    except IndexError:
        return {}
    except:
        raise InternalError('Failed to load local state from the sawtooth state')


def _set_state_data(imsi, state, context):
    address = make_ccellular_address(imsi)
    encoded = cbor.dumps(state)
    addresses = context.set_state({address: encoded})
    if not addresses:
        raise InternalError('Failed to set the local state of the sawtooth node')


def _do_ccellular(action, imsi, value, state, database):
    if action == 'set':
        message = 'Setting {} to {}'.format(imsi, value)
        # Value is the serialized protobuf message which needs to be deserialized before being sent to MongoDB
        proto_message = deserialize_proto_database_instructions(value)
        print(message)
        if imsi in state:
            raise InvalidTransaction('IMSI {} already exists with value {}'.format(imsi, state[imsi]))
        # Update the Sawtooth Merkle Database
        updated = dict(state.items())
        updated[imsi] = value
        # Update the corresponding MongoDB database of the client
        database.operate(proto_message)
        return updated
    raise InternalError('Invalid function requested to be executed by CCellular Handler')