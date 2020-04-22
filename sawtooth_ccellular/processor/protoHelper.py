from sawtooth_ccellular.processor.utils import encode_utf8string_to_hex_string, decode_hex_string_to_utf8string, \
    deserialize_mongodb_cursor
from sawtooth_ccellular.structures import structures_pb2


def create_database_instruction_message_insert(object_dict):
    # Create the Protobuf object for DatabaseInstruction
    return database_dict_to_proto(object_dict, structures_pb2.DatabaseData.Operation.INSERT)

def create_database_instruction_message_update(object_dict):
    # Create the Protobuf object for DatabaseInstruction
    return database_dict_to_proto(object_dict, structures_pb2.DatabaseData.Operation.UPDATE)

# Takes a dictionary and operation type and converts to a proto message
def database_dict_to_proto(object_dict, operation):
    # Create a message and pull out important bits
    proto_message = structures_pb2.DatabaseData()
    proto_message.operation = operation
    proto_message.imsi = object_dict.get("imsi", "") or ""
    proto_message.remote = object_dict.get("remote", "") or False
    proto_message.ownership = object_dict.get("ownership", "") or ""
    security = object_dict.get("security", "")
    if security:
        proto_message.k = security.get("k") or ""
        proto_message.amf = security.get("amf") or ""
        proto_message.op = security.get("op") or ""
        proto_message.opc = security.get("opc") or ""
    return proto_message

# Takes a proto message returns a dictionary
def database_proto_to_dict(proto_message):
    object_dict = {'imsi' : proto_message.imsi or "",
                   'remote' : proto_message.remote or False,
                   'ownership' : proto_message.ownership or "",
                   'security' : {
                       'k' : proto_message.k or "",
                       'amf' : proto_message.amf or "",
                       'op' : proto_message.op or "",
                       'opc' : proto_message.opc or "",
                       }
                   }
    return object_dict

# Takes a dict (i.e. op_document) and strips for useful info
def database_dict_strip(object_dict):
    new_obj = {'imsi' : object_dict.get('imsi') or "",
                #    'remote' : object_dict.get('remote') or False,
                #    'ownership' : object_dict.get('ownership') or "",
                   'security' : {
                       'k' : object_dict.get('k') or "",
                       'amf' : object_dict.get('amf') or "",
                       'op' : object_dict.get('op') or "",
                       'opc' : object_dict.get('opc') or "",
                       }
                   }
    return new_obj

def read_database_instruction_message_proto(proto_hex_instruction):
    # Returns a serialized MongoDB Cursor
    ser_database_instruction = decode_hex_string_to_utf8string(proto_hex_instruction)
    # Convert serialized instruction to Mongo Instruction
    database_instruction = deserialize_mongodb_cursor(ser_database_instruction)
    return database_instruction
