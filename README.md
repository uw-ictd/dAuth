# dAuth - Distrbuted LTE Authentication

### Requirements

1. Have a `Hyperledger Sawtooth` network up and running which are connected to the `nextepc` distribution.
2. Configure and deploy the network correctly.

> This module ships with a default test single node sawtooth docker instance as well as a bunch of nodes running the
> PoET consensus. Each `nextepc` installation in the Community Cellular installation should generate a 
> key pair and attach to the network.

---

### Common Issues [WIP]

This library depends on the crypto requirements from `libsecp256k1` which can be installed by doing:

```bash
$ sudo apt install libsecp256k1-dev
```

If there are additional issues while building the codebase which result in the error

```bash
Setup script exited with 'pkg-config' is required to install this package.
```

You might need the corresponding build tools to get the build for the `secp256k1`. This can be installed by doing

```bash
$ sudo apt install build-essential automake pkg-config libtool libffi-dev libgmp-dev
```
