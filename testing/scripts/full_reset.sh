# Run this from top level directory
# i.e. ./testing/scripts/full_reset.sh

# The following commands will reset all state and restore default configs

# Stop services to prevent more communication
python3 ./testing/scripts/dauth_service.py -d . -n colte1 colte2 colte3 colte4 --stop-service

# Upload default configs
python3 ./testing/scripts/dauth_service.py -d . -n colte1 --upload-config ./configs/dauth-service/sample1.yaml
python3 ./testing/scripts/dauth_service.py -d . -n colte2 --upload-config ./configs/dauth-service/sample2.yaml
python3 ./testing/scripts/dauth_service.py -d . -n colte3 --upload-config ./configs/dauth-service/sample3.yaml
python3 ./testing/scripts/dauth_service.py -d . -n colte4 --upload-config ./configs/dauth-service/sample4.yaml

# Reset the directory service first
python3 ./testing/scripts/dauth_directory.py -d . -n directory --reset-service

# Finally, reset all dauth services
python3 ./testing/scripts/dauth_service.py -d . -n colte1 colte2 colte3 colte4 --reset-service
