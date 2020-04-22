from pymongo import MongoClient

from sawtooth_ccellular.processor.constants import DB_CONNECTION_HOST, DB_CONNECTION_PORT, DB_DATABASE_NAME, \
    DB_COLLECTION_NAME, PROCESSOR_NAME
from sawtooth_ccellular.processor.protoHelper import read_database_instruction_message_proto
from sawtooth_ccellular.structures import structures_pb2

from bson.objectid import ObjectId


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
        if proto_database_operation.operation == structures_pb2.DatabaseInstruction.Operation.UPDATE:
            hex_encoded_instruction = proto_database_operation.hex_encoded_object
            db_instruction = read_database_instruction_message_proto(hex_encoded_instruction)
            if db_instruction['initiated'] == PROCESSOR_NAME:
                print("I am the person who initiated this. Do not replay this")
            else:
                print("[Update] Updating the local database with instruction details")
                print(db_instruction)
                try:
                    object_queried = db_instruction['o2']
                    execute_instruction = db_instruction['o']
                    if '$set' in execute_instruction:
                        execute_instruction.pop('$v')
                    if '$unset' in execute_instruction:
                        execute_instruction.pop('$v')
                    cursor = self.db[DB_COLLECTION_NAME].find(object_queried)
                    print("Updated the database from the instruction received from Sawtooth")
                    self.db[DB_COLLECTION_NAME].update(object_queried, execute_instruction)
                except:
                    print("Failed to perform database insert")