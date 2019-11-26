import argparse
import pkg_resources

from sawtooth_ccellular.processor.constants import DISTRIBUTION_NAME, DEFAULT_URL


def parse_args(args):
    parser = argparse.ArgumentParser(formatter_class=argparse.RawTextHelpFormatter)

    parser.add_argument('-C', '--connect', default=DEFAULT_URL, help='TCP Endpoint for the validator connection')
    parser.add_argument('-v', '--verbose', action='count', default=0, help='Verbosity Level for the logging')

    try:
        version = pkg_resources.get_distribution(DISTRIBUTION_NAME).version
    except pkg_resources.DistributionNotFound:
        version = 'UNKNOWN'

    parser.add_argument('-V', '--version', action='version',
                        version=(DISTRIBUTION_NAME + ' (Hyperledger Sawtooth) version {}').format(version),
                        help='print version information')

    return parser.parse_args(args)
