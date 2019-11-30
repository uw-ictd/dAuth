from bson import json_util
from sawtooth_ccellular.structures import structures_pb2


def encode_utf8string_to_hex_string(input_string):
    """
    Always encode an UTF-8 String to a corresponding Hex Format before any transmissions through the network.
    :param input_string: A UTF-8 encoded string
    :return: A String encoded in a hex format.
    """
    return input_string.encode().hex()


def decode_hex_string_to_utf8string(hex_string):
    """
    Use this method to decode a hex string into a corresponding UTF-8 string which can be processed further.
    :param hex_string: A hex encoded string
    :return: A string in UTF-8
    """
    return bytes.fromhex(hex_string).decode('utf-8')


def serialize_mongodb_cursor(cursor_object):
    return json_util.dumps(cursor_object)


def deserialize_mongodb_cursor(serialized_cursor):
    return json_util.loads(serialized_cursor)


def serialize_proto_database_instruction(instruction):
    # TODO: Perform basic sanity checks before serializing to bytes
    return instruction.SerializeToString()


def deserialize_proto_database_instructions(serialized_instruction):
    instruction = structures_pb2.DatabaseInstruction()
    instruction.ParseFromString(serialized_instruction)
    return instruction
