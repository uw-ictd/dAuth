import os
import sys

from sawtooth_ccellular.client.action_manager import do_set, do_get
from sawtooth_ccellular.client.parser import create_parser


def main(prog_name=os.path.basename(sys.argv[0]), args=None):
    if args is None:
        args = sys.argv[1:]
    parser = create_parser(prog_name)
    args = parser.parse_args(args)

    if not args.command:
        parser.print_help()
        sys.exit(1)

    if args.command == 'set':
        do_set(args)
    elif args.command == 'get':
        do_get(args)
    else:
        print("This operation is not yet supported. Please raise a feature request.")


def main_wrapper():
    try:
        main()
    except BaseException as e:
        print("Error: {}".format(e), file=sys.stderr)
        sys.exit(1)
    except:
        sys.exit(1)