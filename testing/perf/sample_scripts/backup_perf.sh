
NAME="sample_backup_perf_test"
CONFIG="./testing/perf/sample_configs/vagrant.yaml"

mkdir -p ./out

for NUM_UES in 50 100  # Can add more UEs to this list
do
    echo "Running" "$NAME" "with $NUM_UES UEs in 30s"
    python3 ./testing/perf/run_perf.py \
    -p "$CONFIG" \
    -c ./testing/configs \
    -u ./testing/perf/ue_driver.py \
    -d . \
    -n "$NUM_UES" \
    -i 30000 \
    -t 0 \
    -k 2 \
    --backup-auth \
    --debug \
    >> ./out/"$NAME".out \
    2>> ./out/"$NAME".log
done
