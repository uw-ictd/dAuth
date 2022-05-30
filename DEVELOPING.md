# dAuth High-Level Dev Flow

dAuth has three main components, a local service (services/dauth-service) which
holds authentication state and can coordinate with other dAuth nodes via gRPC, a
directory service for looking up other dAuth nodes, and a modified SEAF, based
on the open5gs ausf.

Currently the SEAF communicates with the dauth-service via a minimalistic gRPC
interface, and expects the dauth-service to be running at its default port.

# Deploying to the test environment for iterative development

There is a convenience script, deploy.py, in the `infra/` directory which can be
used to build and copy the required components into the test VMs colte1 and
colte2. It is designed to run within the dauthDev vm. The first time you run it,
you will want to build and deploy everything, like so:

```bash
cd infra
poetry install
poetry run python3 deploy.py --build-dauth --build-open5gs --deploy-dauth --deploy-open5gs --dest-host colte1.local --dest-host colte2.local
```

Building the packages for open5gs is expensive since they're build from scratch
via the debian process, so after deploying open5gs, you probably just want to
selectively build and deploy dauth with:

```bash
poetry install
poetry run python3 deploy.py --build-dauth --deploy-dauth --dest-host colte1.local --dest-host colte2.local
```

This can be shortened to `deploy.py -bd -o ${Host} -o ${Host2}`

This script replaces some of the more tedious build, deploy, and run steps in
the manual process below.

All deployed services will be running via systemd, so use the usual `systemctl
${CMD} dauth.service open5gs-ausfd.service` and `journalctl -f -u dauth.service
-u open5gs-ausfd.service` to control them and get their logs.

# Manually running an end-to-end test

**For this test, you only need the ueransim and dauthDev vms**

Several steps are currently required to run dAuth. This should hopefully be
streamlined and simplified over the next few weeks. All steps take place in the dauthDev vm in separate terminals unless otherwise noted!

1. Build and run the manager service

```bash
cd services
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
