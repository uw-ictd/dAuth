import sys

from sawtooth_sdk.processor.core import TransactionProcessor
from sawtooth_sdk.processor.log import init_console_logging

from sawtooth_ccellular.processor.handler import CCellularTransactionHandler
from sawtooth_ccellular.processor.nextepc_db_handler import NextEPCHandler
from sawtooth_ccellular.processor.parser import parse_args


def main(args=None):
    if args is None:
        args = sys.argv[1:]
    opts = parse_args(args)
    print("[START] Running the Community Cellular Transaction Processor.")
    processor = None
    nextepc_handler = None
    try:
        processor = TransactionProcessor(url=opts.connect)
        nextepc_handler = NextEPCHandler()
        init_console_logging(verbose_level=opts.verbose)
        handler = CCellularTransactionHandler()
        processor.add_handler(handler)
        processor.start()
    except KeyboardInterrupt:
        print("[STOP] Received INT to stop the Community Cellular Transaction Processor")
        pass
    except Exception as e:
        print("Error while running Community Cellular Transaction Processor: {}".format(e), file=sys.stderr)
    finally:
        if nextepc_handler is not None:
            nextepc_handler.close()
        if processor is not None:
            processor.stop()
