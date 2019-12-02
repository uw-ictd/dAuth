## Deployment

Each node has running tmux sessions which run the 

1. `mongodb` instances in replicated set mode on the same machine on ports 27017 and 27018
2. `open5gs-*` services
3. `<repo>/data/db` for the main database and `<repo>/data/replica` as the replica.
4. Sawtooth node running as a service and the corresponding services needing to be packaged into a service


### Setup

- [x] Configured correctly on `colte-1`
    - [x] EPC
    - [x] MongoDB Master/Replica
    - [x] Sawtooth
    - [x] Running and connecting `ccellular-tp`
- [x] Configured correctly on `colte-2`
    - [x] EPC
    - [x] MongoDB Master/Replica
    - [x] Sawtooth
    - [x] Running and connecting `ccellular-tp`
- [x] Configured correctly on `colte-3`
    - [x] Sawtooth
    - [x] Running and connecting `ccellular-tp`
    
- [x] Ensure that the key pairs on all the machines are located at the path corresponding to:
    - [x] `user$ ~/.sawtooth/keys/<user>.priv` for the user running the `ccellular-tp` process.
