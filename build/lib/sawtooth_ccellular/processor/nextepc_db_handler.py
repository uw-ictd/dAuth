from pymongo import MongoClient
from mongotriggers import MongoTrigger

from sawtooth_ccellular.processor.handler import _sha512

from sawtooth_ccellular.client.action_manager import _get_client
from sawtooth_ccellular.client.ccellular_client import CCellularClient
from sawtooth_ccellular.processor.constants import DB_CONNECTION_HOST, DB_CONNECTION_PORT, DB_DATABASE_NAME, \
    DB_COLLECTION_NAME, CLIENT_DEFAULT_URL, CLIENT_DEFAULT_KEY_PATH, PROCESSOR_NAME
from sawtooth_ccellular.processor.protoHelper import create_database_instruction_message_insert, \
    create_database_instruction_message_update
from sawtooth_ccellular.processor.utils import serialize_mongodb_cursor, serialize_proto_database_instruction


class NextEPCHandler:
    client = None
    db = None
    triggers = None

    trigger_buffer = None

    def __init__(self):
        client = MongoClient(DB_CONNECTION_HOST, DB_CONNECTION_PORT)
        db = client[DB_DATABASE_NAME]
        self.db = db
        self.client = client
        print(db.name, " is the database being used.")
        triggers = MongoTrigger(self.client)
        print("[Connector] Registering a trigger monitor for the Insert Operations on the database")
        print("[Connector][OK] Registering a trigger monitor for the Insert Operations on the database")
        print("[Connector] Registering a trigger monitor for the Delete Operations on the database")
        print("[Connector][OK] Registering a trigger monitor for the Delete Operations on the database")
        triggers.register_op_trigger(self._notify_operation_changes, DB_DATABASE_NAME, DB_COLLECTION_NAME)
        print("[Connector] Starting to watch the status of the database on the replica set")
        triggers.tail_oplog()
        print("[Connector][OK] Starting to watch the status of the database on the replica set")
        self.triggers = triggers
        self.trigger_buffer = {}

    def close(self):
        print("[Connector] Releasing all the locks and stopping watchers from watching the MongoDB Database")
        self.triggers.stop_tail()
        print("[Connector][OK] Releasing all the locks and stopping watchers from watching the MongoDB Database")

    def _notify_operation_changes(self, op_document):
        if op_document['op'] == 'u':
            self._notify_changes_to_mongo_update(op_document)
        if op_document['op'] == 'i':
            self._notify_changes_to_mongo_insert(op_document)
        if op_document['op'] == 'd':
            self._notify_changes_to_mongo_delete(op_document)

    def _notify_changes_to_mongo_update(self, op_document):
        object_queried = op_document['o2']
        operation = op_document['o']
        print("[Update] : ", serialize_mongodb_cursor(op_document))
        self._db_handle_update(object_queried, operation, op_document)

    def _notify_changes_to_mongo_insert(self, op_document):
        object_modified = op_document['o']
        print("[Insert] : ", serialize_mongodb_cursor(object_modified))
        self._db_handle_find_and_insert(object_modified)

    def _notify_changes_to_mongo_delete(self, op_document):
        object_modified = op_document['o']
        print("[Delete] : ", serialize_mongodb_cursor(object_modified))
        self._db_handle_delete(object_modified)

    def _db_handle_find_and_insert(self, object_dict):
        if 'imsi' in object_dict:
            query = {'imsi': object_dict['imsi']}
            cursor = self.db[DB_COLLECTION_NAME].find(query)
            for data_collection in cursor:
                if data_collection['imsi'] == object_dict['imsi']:
                    print("[DATABASE][KEY EXISTS] : The value of the IMSI already exists. I am the initiator")
                    if 'remote' in object_dict:
                        # Don't do anything anymore. The instruction has been inserted from Transaction Processors side
                        print("[TPINSERT] Receiver node replaying instructions. Avoid reissuing a sawtooth transaction")
                        # This message was received from another node. Do not retry a commit.
                    else:
                        object_dict['remote'] = True  # Use this boolean to ensure that message initiator is this HSS
                        object_dict['ownership'] = PROCESSOR_NAME
                        serialized_cursor = serialize_mongodb_cursor(object_dict)
                        proto_message = create_database_instruction_message_insert(serialized_cursor)
                        serialized_proto_message = serialize_proto_database_instruction(proto_message)
                        # Set an instruction to add value to the HSS
                        # Run the equivalent of `ccellular set <IMSI> <serialized_proto_message>`
                        ccellular_client = _get_client(None)
                        ccellular_client.set(object_dict['imsi'], serialized_proto_message)
                else:
                    print("Inserting the value to the {} in collection {}".format(DB_DATABASE_NAME, DB_COLLECTION_NAME))
                    self.db[DB_COLLECTION_NAME].insert_one(object_dict)
        else:
            print("[ERROR] Malformed Request. Cannot process this anymore.")

    def _db_handle_update(self, object_queried, modification_to_apply, object_dict):
        cursor = self.db[DB_COLLECTION_NAME].find(object_queried)
        for data_collection in cursor:
            imsi_modified = data_collection['imsi']
            execute_instruction = modification_to_apply
            if 'ownership' in data_collection:
                if data_collection['ownership'] != 'peer':
                    if '$set' in modification_to_apply:
                        execute_instruction.pop('$v')
                    if '$unset' in modification_to_apply:
                        execute_instruction.pop('$v')
                    self.db[DB_COLLECTION_NAME].update({'imsi': imsi_modified}, execute_instruction)
                    # This is available on the remote HSS nodes which receive this operation and executes it
                    print("[Update] Successfully updated {} with required updates".format(imsi_modified))
            else:
                # This means that the HSS is the initiator since it has no ownership
                print("Making a transaction for setting the update instruction")
                object_dict['initiated'] = PROCESSOR_NAME
                serialized_cursor = serialize_mongodb_cursor(object_dict)
                proto_message = create_database_instruction_message_update(serialized_cursor)
                serialized_proto_message = serialize_proto_database_instruction(proto_message)
                ccellular_client = _get_client(None)
                ccellular_client.set(imsi_modified, serialized_proto_message)

    def _db_handle_delete(self, object_key_id):
        query = object_key_id
        cursor = self.db[DB_COLLECTION_NAME].find(query)
        for data_collection in cursor:
            if 'imsi' in data_collection:
                # TODO: Add more logic here for deletion of an item from the HSS database of an EPC
                self.db[DB_COLLECTION_NAME].delete_one(query)
            else:
                print("[ERROR] Never had information about this object to proceed with deletion. Silently passing")

