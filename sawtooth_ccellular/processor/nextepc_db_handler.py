from pymongo import MongoClient
from mongotriggers import MongoTrigger

from sawtooth_ccellular.processor.handler import _sha512

from sawtooth_ccellular.client.action_manager import _get_client
from sawtooth_ccellular.client.ccellular_client import CCellularClient
from sawtooth_ccellular.processor.protoHelper import create_database_instruction_message_insert, \
    create_database_instruction_message_update, database_dict_strip
from sawtooth_ccellular.processor.utils import serialize_mongodb_cursor, serialize_proto_database_instruction
from sawtooth_ccellular.processor.global_variables import GlobalVariables as gvars


# Handles the triggers on mongodb
class NextEPCHandler:
    client = None
    db = None
    triggers = None

    trigger_buffer = None

    def __init__(self):
        client = MongoClient(gvars.DB_CONNECTION_HOST, gvars.DB_CONNECTION_PORT)
        db = client[gvars.DB_DATABASE_NAME]
        self.db = db
        self.client = client
        gvars.LOGGER.log('NextEPCHandler', "__init__ -- db is " + str(db.name))
        print(db.name, " is the database being used.")
        triggers = MongoTrigger(self.client)
        print("[Connector] Registering a trigger monitor for the Insert Operations on the database")
        print("[Connector][OK] Registering a trigger monitor for the Insert Operations on the database")
        print("[Connector] Registering a trigger monitor for the Delete Operations on the database")
        print("[Connector][OK] Registering a trigger monitor for the Delete Operations on the database")
        triggers.register_op_trigger(self._notify_operation_changes, gvars.DB_DATABASE_NAME, gvars.DB_COLLECTION_NAME)
        print("[Connector] Starting to watch the status of the database on the replica set")
        triggers.tail_oplog()
        print("[Connector][OK] Starting to watch the status of the database on the replica set")
        gvars.LOGGER.log('NextEPCHandler', "__init__ -- finished")
        self.triggers = triggers
        self.trigger_buffer = {}

    def close(self):
        print("[Connector] Releasing all the locks and stopping watchers from watching the MongoDB Database")
        self.triggers.stop_tail()
        print("[Connector][OK] Releasing all the locks and stopping watchers from watching the MongoDB Database")

    # Triggered on new db operation
    def _notify_operation_changes(self, op_document):
        gvars.LOGGER.log('NextEPCHandler', "_notify_operation_changes -- triggered with " + op_document['op'])
        gvars.LOGGER.log('NextEPCHandler', "imsi: " + str(op_document['o'].get('imsi')))
        gvars.LOGGER.log('NextEPCHandler', "security: " + str(op_document['o'].get('security')))
        if op_document['op'] == 'u':
            self._notify_changes_to_mongo_update(op_document)
        if op_document['op'] == 'i':
            self._notify_changes_to_mongo_insert(op_document)
        if op_document['op'] == 'd':
            self._notify_changes_to_mongo_delete(op_document)

    # Operation functions for update/insert/delete
    def _notify_changes_to_mongo_update(self, op_document):
        object_queried = op_document['o2']
        operation = op_document['o']
        print("[Update] : ", serialize_mongodb_cursor(op_document))
        gvars.LOGGER.log('NextEPCHandler', "_notify_changes_to_mongo_update -- " + str(serialize_mongodb_cursor(op_document)))
        self._db_handle_update(object_queried, operation, op_document)

    def _notify_changes_to_mongo_insert(self, op_document):
        object_modified = op_document['o']
        print("[Insert] : ", serialize_mongodb_cursor(object_modified))
        gvars.LOGGER.log('NextEPCHandler', "_notify_changes_to_mongo_insert -- " + str(serialize_mongodb_cursor(object_modified)))
        self._db_handle_find_and_insert(object_modified)

    def _notify_changes_to_mongo_delete(self, op_document):
        object_modified = op_document['o']
        print("[Delete] : ", serialize_mongodb_cursor(object_modified))
        gvars.LOGGER.log('NextEPCHandler', "_notify_changes_to_mongo_delete -- " + str(serialize_mongodb_cursor(object_modified)))
        self._db_handle_delete(object_modified)

    def _db_handle_find_and_insert(self, object_dict):
        # Receiving object dict from database operation
        gvars.LOGGER.log("NextEPCHandler", "_db_handle_find_and_insert")

        if 'imsi' in object_dict:
            # Try to find a matching entry in the local/current database
            new_obj_dict = database_dict_strip(object_dict)
            query = {'imsi': new_obj_dict['imsi']}
            cursor = self.db[gvars.DB_COLLECTION_NAME].find(query)
            i = 0
            for data_collection in cursor:
                if data_collection['imsi'] == object_dict['imsi']:
                    i += 1
                    gvars.LOGGER.log("NextEPCHandler", "_db_handle_find_and_insert -- imsi exists: " + str(new_obj_dict['imsi']))
                    print("[DATABASE][KEY EXISTS] : The value of the IMSI already exists. I am the initiator")

                    # Check if this is a remote operation
                    # If it is, it was handled in the transaction processor
                    if object_dict.get('remote'):
                        # Don't do anything anymore. The instruction has been inserted from Transaction Processors side
                        print("[TPINSERT] Receiver node replaying instructions. Avoid reissuing a sawtooth transaction")
                        # This message was received from another node. Do not retry a commit.
                        gvars.LOGGER.log("NextEPCHandler", "_db_handle_find_and_insert -- received from another node, doing nothing")

                    # Not a remote transaction, so this has not been propigated yet
                    else:
                        # Set owner and remote
                        new_obj_dict['remote'] = True  # Use this boolean to ensure that message initiator is this HSS
                        new_obj_dict['ownership'] = gvars.PROCESSOR_NAME

                        # Create and send out the transaction
                        proto_message = create_database_instruction_message_insert(new_obj_dict)

                        # Make sure to test with real size values
                        serialized_proto_message = serialize_proto_database_instruction(proto_message)  # Compare with old message for sizes
                        # Set an instruction to add value to the HSS
                        # Run the equivalent of `ccellular set <IMSI> <serialized_proto_message>`
                        gvars.LOGGER.log("NextEPCHandler", "_db_handle_find_and_insert -- using ccellular client to set")
                        ccellular_client = _get_client(None)
                        ccellular_client.set(object_dict['imsi'], serialized_proto_message)
                else:
                    print("Inserting the value to the {} in collection {}".format(gvars.DB_DATABASE_NAME, gvars.DB_COLLECTION_NAME))
                    gvars.LOGGER.log("NextEPCHandler", "Inserting the value to the {} in collection {}".format(gvars.DB_DATABASE_NAME, gvars.DB_COLLECTION_NAME))
                    self.db[gvars.DB_COLLECTION_NAME].insert_one(object_dict)

            print("NUM OPERATIONS:", i)
        else:
            print("[ERROR] Malformed Request. Cannot process this anymore.")
            gvars.LOGGER.log("NextEPCHandler", "_db_handle_find_and_insert -- bad request")

    def _db_handle_update(self, object_queried, modification_to_apply, object_dict):
        gvars.LOGGER.log("NextEPCHandler", "_db_handle_update")
        cursor = self.db[gvars.DB_COLLECTION_NAME].find(object_queried)
        for data_collection in cursor:
            imsi_modified = data_collection['imsi']
            execute_instruction = modification_to_apply
            if 'ownership' in data_collection:
                if data_collection['ownership'] != 'peer':
                    if '$set' in modification_to_apply:
                        execute_instruction.pop('$v')
                    if '$unset' in modification_to_apply:
                        execute_instruction.pop('$v')
                    self.db[gvars.DB_COLLECTION_NAME].update({'imsi': imsi_modified}, execute_instruction)
                    # This is available on the remote HSS nodes which receive this operation and executes it
                    print("[Update] Successfully updated {} with required updates".format(imsi_modified))
            else:
                # This means that the HSS is the initiator since it has no ownership
                print("Making a transaction for setting the update instruction")
                object_dict['initiated'] = gvars.PROCESSOR_NAME
                serialized_cursor = serialize_mongodb_cursor(object_dict)
                proto_message = create_database_instruction_message_update(serialized_cursor)
                serialized_proto_message = serialize_proto_database_instruction(proto_message)
                ccellular_client = _get_client(None)
                ccellular_client.set(imsi_modified, serialized_proto_message)

    def _db_handle_delete(self, object_key_id):
        gvars.LOGGER.log("NextEPCHandler", "_db_handle_delete")
        query = object_key_id
        cursor = self.db[gvars.DB_COLLECTION_NAME].find(query)
        for data_collection in cursor:
            if 'imsi' in data_collection:
                # TODO: Add more logic here for deletion of an item from the HSS database of an EPC
                self.db[gvars.DB_COLLECTION_NAME].delete_one(query)
            else:
                print("[ERROR] Never had information about this object to proceed with deletion. Silently passing")

