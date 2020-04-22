from pymongo import MongoClient

from sawtooth_ccellular.processor.global_variables import GlobalVariables as gvars
from sawtooth_ccellular.processor.protoHelper import database_proto_to_dict
from sawtooth_ccellular.structures import structures_pb2

from bson.objectid import ObjectId


class DatabaseManager:
    client = None
    db = None

    def __init__(self):
        client = MongoClient(gvars.DB_CONNECTION_HOST, gvars.DB_CONNECTION_PORT)
        db = client[gvars.DB_DATABASE_NAME]
        self.db = db
        self.client = client
        print("[DATABASE MANAGER] Database Manager is connected to {}".format(gvars.DB_DATABASE_NAME))

    # Used when receiving from another node
    def operate(self, proto_database_operation):
        """
        We expect a protobuf deserialized DatabaseInstruction here
        :param proto_database_operation:
        :return:
        """
        # TODO: Change what the operation does to reflect the new proto message format
        # TODO TODO TODO
        # Check for the key existing
        if proto_database_operation.operation == structures_pb2.DatabaseData.Operation.INSERT:
            print(proto_database_operation)
            database_operation = database_proto_to_dict(proto_database_operation)
            print("[INSERT] Updating local database with details received in the instruction")
            try:
                if database_operation["ownership"] != gvars.PROCESSOR_NAME:
                    self.db[gvars.DB_COLLECTION_NAME].insert_one(database_operation)
            except Exception as e:
                print("FAILED:", str(e))
                pass

        if proto_database_operation.operation == structures_pb2.DatabaseData.Operation.UPDATE:
            hex_encoded_instruction = proto_database_operation.hex_encoded_object
            db_instruction = read_database_instruction_message_proto(hex_encoded_instruction)
            if db_instruction['initiated'] == gvars.PROCESSOR_NAME:
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
                    cursor = self.db[gvars.DB_COLLECTION_NAME].find(object_queried)
                    print("Updated the database from the instruction received from Sawtooth")
                    self.db[gvars.DB_COLLECTION_NAME].update(object_queried, execute_instruction)
                except:
                    print("Failed to perform database insert")