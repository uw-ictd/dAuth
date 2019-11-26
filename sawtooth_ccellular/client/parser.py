import argparse

import pkg_resources

from sawtooth_ccellular.client.constants import DISTRIBUTION_NAME
from sawtooth_ccellular.client.constants import GET_MESSAGE, SET_MESSAGE


def create_parent_parser(prog_name):
    parent_parser = argparse.ArgumentParser(prog=prog_name, add_help=False)
    parent_parser.add_argument('-v', '--verbose', action='count', help='Enable more verbose output of the logs')
    try:
        version = pkg_resources.get_distribution(DISTRIBUTION_NAME).version
    except pkg_resources.DistributionNotFound:
        version = 'UNKNOWN'

    parent_parser.add_argument('-V', '--version', action='version',
                               version=(DISTRIBUTION_NAME + '(Hyperledger Sawtooth version {})').format(version),
                               help='Display version information')
    return parent_parser


def add_set_parser(subparsers, parent_parser):
    message = SET_MESSAGE
    parser = subparsers.add_parser('set', parents=[parent_parser], description=message, help='Sets the AVs for IMSI')
    parser.add_argument('name', type=str, help='IMSI Value to set the AVs to')
    # TODO: Convert the value to be a list taking `nargs[+]` here.
    parser.add_argument('value', type=str, help='Authentication vectors encoded into a specific format')
    # TODO: Add the ability to post this transaction to a different URL processor / REST API endpoint


def add_get_parser(subparsers, parent_parser):
    message = GET_MESSAGE
    parser = subparsers.add_parser('get', parents=[parent_parser], description=message, help='Gets the AVs for IMSI')
    parser.add_argument('name', type=str, help='IMSI Value to Query for AVs')
    # TODO: Add the ability to read this transaction from a different URL processor / REST API endpoint.


def create_parser(prog_name):
    parent_parser = create_parent_parser(prog_name)
    parser = argparse.ArgumentParser(parents=[parent_parser], formatter_class=argparse.RawDescriptionHelpFormatter)
    subparsers = parser.add_subparsers(title='subcommands', dest='command')
    add_set_parser(subparsers, parent_parser)
    add_get_parser(subparsers, parent_parser)

    # Add more client side parsers here as required. These are the parsers for each of the sub commands
    return parser

