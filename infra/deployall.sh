# Selectively rebuild dauth since its relatively fast
poetry run python3 deploy.py --build-dauth --deploy-dauth --deploy-open5gs --dest-host colte1.local --dest-host colte2.local --dest-host colte3.local --dest-host colte4.local
poetry run python3 deploy.py --deploy-ueransim --dest-host ueransim.local
poetry run python3 deploy.py --deploy-dauth-directory --dest-host directory.local
