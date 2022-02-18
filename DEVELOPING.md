# dAuth High-Level Dev Flow

dAuth has two main components, a manager which holds authentication state and
can coordinate with over dAuth nodes via gRPC, and a modified SEAF, based on the
open5gs ausf.

Currently the SEAF communicates with the manager via a minimalistic gRPC
interface, and expects the manager to be running at its default port.

# Running an end-to-end test

**For this test, you only need the ueransim and dauthDev vms**

Several steps are currently required to run dAuth. This should hopefully be
streamlined and simplified over the next few weeks. All steps take place in the dauthDev vm in separate terminals unless otherwise noted!

1. Build and run the manager service

```bash
cd manager-rs
cargo build
./target/debug/dauth-service configs/basic-ueransim-home-attach.yml
```

2. Build open5gs and run the modified ausf (representing our SEAF)

```bash
cd open5gs
meson build
ninja -C build
sudo ./build/src/ausf/open5gs-ausfd -c configs/dauth/ausf.yaml
```

3. Run the modified UDM (built in the previous step)

```bash
cd open5gs
sudo ./build/src/udm/open5gs-udmd -c configs/dauth/udm.yaml
```

4. Install and run other unmodified core network components. They should be
   running automatically via systemd after installation.

```bash
sudo apt install open5gs-upf open5gs-smf open5gs-amf open5gs-nrf open5gs-udr open5gs-pcf open5gs-nssf open5gs-bsf
```

5. Configure access to core network from the VM subnet
* Update AMF config to listen for NGAP on an exposed IP

`/etc/open5gs/amf.yaml`
```yaml
amf:
    ngap:
        - addr: 192.168.56.2
```
* Update UPF config to listen for gtp-u on an exposed IP

`/etc/open5s/upf.yaml`
```yaml
upf:
   gtpu:
      - addr: 192.168.56.2
```

* Restart the upf and amf
```bash
sudo systemctl restart open5gs-amfd open5gs-smfd
```

6. Add network slice info to UDR (eventually can be removed and replaced with a
   default level of service)
```bash
cd infra
./open5gs_dbconf.sh add 901700000000001 10.45.0.2 aaa bbb internet
```

7. In separate terminals **in the ueransim VM**
* start the gNodeB
```bash
./UERANSIM/build/nr-gnb -c configs/ueransim/gnb-1.yaml
```
* start the UE
```bash
sudo ./UERANSIM/build/nr-ue -c configs/ueransim/ue.yaml
```
