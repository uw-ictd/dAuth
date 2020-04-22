import argparse
import pkg_resources

from sawtooth_ccellular.processor.global_variables import GlobalVariables as gvars


def parse_args(args):
    parser = argparse.ArgumentParser(formatter_class=argparse.RawTextHelpFormatter)

    parser.add_argument('-C', '--connect', default=gvars.DEFAULT_URL, help='TCP Endpoint for the validator connection')
    parser.add_argument('-v', '--verbose', action='count', default=0, help='Verbosity Level for the logging')
    parser.add_argument('-d', '--dbname', default=gvars.DB_DATABASE_NAME, help='Manually set database name')

    try:
        version = pkg_resources.get_distribution(gvars.DISTRIBUTION_NAME).version
    except pkg_resources.DistributionNotFound:
        version = 'UNKNOWN'

    parser.add_argument('-V', '--version', action='version',
                        version=(gvars.DISTRIBUTION_NAME + ' (Hyperledger Sawtooth) version {}').format(version),
                        help='print version information')

    return parser.parse_args(args)
