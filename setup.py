from __future__ import print_function

import os
import sys
import subprocess
from distutils.spawn import find_executable

from setuptools import setup, find_packages

data_files = []

# if os.path.exists("/etc/default"):
#     data_files.append('etc/default', ['packaging/systemd/sawtooth-ccellular-tp-python'])
#
# if os.path.exists("/lib/systemd/system"):
#     data_files.append('/lib/systemd/system', ['packaging/systemd/sawtooth-ccellular-tp-python.service'])


# Find the Protobuf compiler and build the corresponding protobuf files
protoc = find_executable('protoc')


def generate_proto(source, require=True):
    if not require and not os.path.exists(source):
        return
    execution_command = [protoc, "-I proto --python_out=sawtooth_ccellular/structures ", source]
    if protoc is None:
        sys.stderr.write("Protobuf compiler is not installed and cannot be found. Please install the binary.")
    if subprocess.call(execution_command) != 0:
        sys.exit(-1)


if __name__ == '__main__':
    # if not os.path.exists("structures"):
    #     os.makedirs("structures")
    # generate_proto('./proto/structures')

    setup(
        name='sawtooth-ccellular',
        version=subprocess.check_output(['bin/get_version.py']).decode('utf-8').strip(),
        description='Sawtooth Community Cellular Transaction Family and Processors',
        author='Sudheesh Singanamalla, Esther Jang.',
        author_email='sudheesh@cs.washington.edu, infrared@cs.washington.edu',
        url='https://github.com/uw-ictd/asterales',
        packages=find_packages(),
        install_requires=[
            'bson',
            'requests',
            'protobuf',
            'pymongo',
            'mongotriggers',
            'cbor',
            'sawtooth-sdk',
            'grpcio'
        ],
        data_files=data_files,
        entry_points={
            'console_scripts': [
                'ccellular = sawtooth_ccellular.client.ccellular_cli:main_wrapper',
                'ccellular-tp = sawtooth_ccellular.processor.main:main',
                'logging-server = network.logging_server:run_server'
            ]
        })
