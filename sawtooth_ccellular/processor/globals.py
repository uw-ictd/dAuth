from sawtooth_ccellular.processor import constants as c


class GlobalVariables:
    # All global variables, initialized to constant values
    # Database
    db_database_name = c.DB_DATABASE_NAME

    # Messaging
    messaging_host = c.MESSAGING_HOST
    messaging_port = c.MESSAGING_PORT
    messaging_logfile = c.MESSAGING_LOGFILE 
    messaging_stream_max = c.MESSAGING_STREAM_MAX

    # Initialize global variables using a dict
    # Useful for setting from parametes
    @staticmethod
    def init_globals(global_kv):
        # Set database name
        if "dbname" in global_kv:
            GlobalVariables.db_database_name = global_kv["dbname"]
        
