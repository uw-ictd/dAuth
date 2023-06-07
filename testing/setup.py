from setuptools import setup, find_packages

setup(name='testing',
      version='1.0',
      description='Testing framework for dauth',
      author='Nick Durand',
      packages=find_packages(),
      install_requires=["paramiko", "pyyaml"]
     )