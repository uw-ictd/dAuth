from distutils.core import setup

setup(name='colte-tests',
      version='1.0',
      description='Testing scripts for colte using ueransim.',
      author='Nick Durand',
      packages=['colte_tests'],
      install_requires=["paramiko"]
     )