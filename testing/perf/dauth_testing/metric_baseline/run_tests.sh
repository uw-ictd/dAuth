# mkdir ./testing/perf/dauth_testing/metric_baseline/results/ 2>/dev/null
# rm ./testing/perf/dauth_testing/metric_baseline/results/* 2>/dev/null

# mkdir ./testing/perf/dauth_testing/metric_baseline/logs/ 2>/dev/null
# rm ./testing/perf/dauth_testing/metric_baseline/logs/* 2>/dev/null

for REPEAT in {1..1}
do
  for FILE in ./testing/perf/dauth_testing/metric_baseline/configs/*
  do
    NAME="$(basename $FILE .yaml)"

    for NUM_UES in 1000
    do
      echo "Running" "$NAME" "with $NUM_UES UEs in 30s"
      python3 ./testing/perf/run_perf.py \
        -p "$FILE" \
        -c ./testing/configs \
        -u ./testing/perf/ue_driver.py \
        -n "$NUM_UES" \
        -i 30000 \
        -t 0 \
        --local-auth \
        --debug \
        >> ./testing/perf/dauth_testing/metric_baseline/results/"$NAME".out \
        2>> ./testing/perf/dauth_testing/metric_baseline/logs/"$NAME".log
    done
  done
done
