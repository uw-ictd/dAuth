from pymongo import MongoClient

from sawtooth_ccellular.processor.constants import DB_CONNECTION_HOST, DB_CONNECTION_PORT, DB_DATABASE_NAME, \
    DB_COLLECTION_NAME
from sawtooth_ccellular.processor.protoHelper import read_database_instruction_message_proto
from sawtooth_ccellular.structures import structures_pb2


class DatabaseManager:
    client = None
    db = None

    def __init__(self):
        client = MongoClient(DB_CONNECTION_HOST, DB_CONNECTION_PORT)
        db = client[DB_DATABASE_NAME]
        self.db = db
        self.client = client
        print("[DATABASE MANAGER] Database Manager is connected to {}".format(DB_DATABASE_NAME))

    def operate(self, proto_database_operation):
        """
        We expect a protobuf deserialized DatabaseInstruction here
        :param proto_database_operation:
        :return:
        """
        if proto_database_operation.operation == structures_pb2.DatabaseInstruction.Operation.INSERT:
            hex_encoded_instruction = proto_database_operation.hex_encoded_object
            db_instruction = read_database_instruction_message_proto(hex_encoded_instruction)
            print("[INSERT] Updating local database with details received in the instruction")
            try:
                self.db[DB_COLLECTION_NAME].insert_one(db_instruction)
            except:
                pass
