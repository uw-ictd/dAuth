# dAuth

dAuth is a research project developed to explore alternative ways to coordinate
multiple cellular networks in a regional federation than traditional roaming. It
uses the concept of a loosely trusted fallback network to increase robustness in
a backwards-compatible manner while being able to detect misbehavior of a subset
of the fallback networks.

# (non) Stability

This codebase is research code, and as such is currently in a proof-of-concept
state that is not appropriate for production use. A non-exhaustive list of
reasons you should not use this code in production without careful evaluation:
* Error handling for non-ideal cases is not robust, and while it should not
  corrupt data, will result in restarts and loss of sessions in a real-world
  network that should otherwise be recoverable.
* There is no native support for handling real-world networking issues like NAT
  and endpoint authentication. The current implementation assumes the endpoint
  identities are verified out of band.
* Cryptography: This code explores a novel way to split-up the burden of
  handling 3GPP-AKA authentication, and while we believe the cryptographic
  libraries used are "correct", we have not validated the code is robust to more
  advanced techniques like side-channel attacks that could result in information
  leakage or compromise in a production use case.

# Running the system

If you would like to run the code and test suites, this repository contains
scripts for starting a small test network of virtual machines with Vagrant which
should allow you to see it in action. You will first have to build the project
in the dauthDev VM, then deploy the project to the other VMs to run the tests.

> TODO(matt9j) Describe the latest build and deploy process
