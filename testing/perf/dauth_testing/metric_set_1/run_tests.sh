mkdir ./testing/perf/dauth_testing/metric_set_1/configs/ 2>/dev/null
mkdir ./testing/perf/dauth_testing/metric_set_1/results/ 2>/dev/null

rm ./testing/perf/dauth_testing/metric_set_1/configs/* 2>/dev/null
rm ./testing/perf/dauth_testing/metric_set_1/results/* 2>/dev/null

python3 ./testing/perf/dauth_testing/metric_set_1/build_configs.py \
        ./testing/perf/dauth_testing/base/ \
        ./testing/perf/dauth_testing/metric_set_1/configs/

for FILE in ./testing/perf/dauth_testing/metric_set_1/configs/*
do
  NAME="$(basename $FILE .yaml)"

  for NUM_UES in 1 5 10 20 50
  do
    echo "Running" "$NAME" "with" "$NUM_UES" "UE(s)"
    python3 ./testing/perf/run_perf.py \
      -p "$FILE" \
      -c ./testing/configs \
      -u ./testing/perf/ue_driver.py \
      -n "$NUM_UES" \
      -i 2000 \
      -t 10 \
      --home-auth \
      --debug \
      >> ./testing/perf/dauth_testing/metric_set_1/results/"$NAME".out
  done
done
