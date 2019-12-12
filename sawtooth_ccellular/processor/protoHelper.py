from sawtooth_ccellular.processor.utils import encode_utf8string_to_hex_string, decode_hex_string_to_utf8string, \
    deserialize_mongodb_cursor
from sawtooth_ccellular.structures import structures_pb2


def create_database_instruction_message_insert(serialized_mongo_cursor):
    hex_encoded_cursor = encode_utf8string_to_hex_string(serialized_mongo_cursor)
    # Create the Protobuf object for DatabaseInstruction
    instruction = structures_pb2.DatabaseInstruction()
    instruction.operation = structures_pb2.DatabaseInstruction.Operation.INSERT
    instruction.hex_encoded_object = hex_encoded_cursor
    return instruction

def create_database_instruction_message_update(serialized_mongo_cursor):
    hex_encoded_cursor = encode_utf8string_to_hex_string(serialized_mongo_cursor)
    # Create the Protobuf object for DatabaseInstruction
    instruction = structures_pb2.DatabaseInstruction()
    instruction.operation = structures_pb2.DatabaseInstruction.Operation.UPDATE
    instruction.hex_encoded_object = hex_encoded_cursor
    return instruction


def read_database_instruction_message_proto(proto_hex_instruction):
    # Returns a serialized MongoDB Cursor
    ser_database_instruction = decode_hex_string_to_utf8string(proto_hex_instruction)
    # Convert serialized instruction to Mongo Instruction
    database_instruction = deserialize_mongodb_cursor(ser_database_instruction)
    return database_instruction
