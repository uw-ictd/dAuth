from __future__ import print_function

import os
import subprocess

from setuptools import setup, find_packages

data_files = []

# if os.path.exists("/etc/default"):
#     data_files.append('etc/default', ['packaging/systemd/sawtooth-ccellular-tp-python'])
#
# if os.path.exists("/lib/systemd/system"):
#     data_files.append('/lib/systemd/system', ['packaging/systemd/sawtooth-ccellular-tp-python.service'])

setup(
    name='sawtooth-ccellular',
    version=subprocess.check_output(['bin/get_version.py']).decode('utf-8').strip(),
    description='Sawtooth Community Cellular Transaction Family and Processors',
    author='Sudheesh Singanamalla, Esther Jang.',
    author_email='sudheesh@cs.washington.edu, infrared@cs.washington.edu',
    url='https://github.com/uw-ictd/asterales',
    packages=find_packages(),
    install_requires=[
        'requests',
        'protobuf',
        'pymongo',
        'cbor',
        'sawtooth-sdk'
    ],
    data_files=data_files,
    entry_points={
        'console_scripts': [
            'ccellular = sawtooth_ccellular.client.ccellular_cli:main_wrapper',
            'ccellular-tp = sawtooth_ccellular.processor.main:main'
        ]
    })
