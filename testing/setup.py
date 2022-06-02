from distutils.core import setup

setup(name='dauth-testing',
      version='1.0',
      description='Testing framework for dauth vms',
      author='Nick Durand',
      packages=['dauth_testing'],
      install_requires=["paramiko"]
     )