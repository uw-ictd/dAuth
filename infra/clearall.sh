#! /usr/bin/env bash

for node in colte1 colte2 colte3 colte4
do
    ssh -i keys/dev_id_ed25519_dauth vagrant@${node}.local "sudo systemctl stop dauth.service"
done

echo "Stopped"

for node in colte1 colte2 colte3 colte4
do
    ssh -i keys/dev_id_ed25519_dauth vagrant@${node}.local "sudo rm /var/lib/dauth/dauth_service/dauth.sqlite3*"
done

echo "DBs Cleared"

for node in colte1 colte2 colte3 colte4
do
    ssh -i keys/dev_id_ed25519_dauth vagrant@${node}.local "sudo systemctl start dauth.service && sudo systemctl restart open5gs-ausfd"
done

echo "Done"
