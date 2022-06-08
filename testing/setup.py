from distutils.core import setup

setup(name='testing',
      version='1.0',
      description='Testing framework for dauth vms',
      author='Nick Durand',
      packages=['testing'],
      package_dir={'testing': '..'},
      install_requires=["paramiko", "pyyaml"]
     )