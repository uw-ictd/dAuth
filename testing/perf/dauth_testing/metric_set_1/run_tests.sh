mkdir ./testing/perf/dauth_testing/metric_set_1/configs/ 2>/dev/null
mkdir ./testing/perf/dauth_testing/metric_set_1/results/ 2>/dev/null

rm ./testing/perf/dauth_testing/metric_set_1/configs/* 2>/dev/null
rm ./testing/perf/dauth_testing/metric_set_1/results/* 2>/dev/null

mkdir ./testing/perf/dauth_testing/metric_set_1/logs/ 2>/dev/null
rm ./testing/perf/dauth_testing/metric_set_1/logs/* 2>/dev/null

python3 ./testing/perf/dauth_testing/metric_set_1/build_configs.py \
        ./testing/perf/dauth_testing/base/ \
        ./testing/perf/dauth_testing/metric_set_1/configs/


for REPEAT in {1..1}
do
  for FILE in ./testing/perf/dauth_testing/metric_set_1/configs/*
  do
    NAME="$(basename $FILE .yaml)"

    for NUM_UES in 10 20 50 75 100 200 300 400 500
    do
      echo "Running" "$NAME" "with $NUM_UES UEs in 30s"
      python3 ./testing/perf/run_perf.py \
        -p "$FILE" \
        -c ./testing/configs \
        -u ./testing/perf/ue_driver.py \
        -n "$NUM_UES" \
        -i 30000 \
        -t 0 \
        --home-auth \
        --debug \
        >> ./testing/perf/dauth_testing/metric_set_1/results/"$NAME".out \
        2>> ./testing/perf/dauth_testing/metric_set_1/logs/"$NAME".log
    done
  done
done
