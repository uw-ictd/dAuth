mkdir ./testing/perf/dauth_testing/metric_set_2/configs/ 2>/dev/null
mkdir ./testing/perf/dauth_testing/metric_set_2/results/ 2>/dev/null

rm ./testing/perf/dauth_testing/metric_set_2/configs/* 2>/dev/null
rm ./testing/perf/dauth_testing/metric_set_2/results/* 2>/dev/null

python3 ./testing/perf/dauth_testing/metric_set_2/build_configs.py \
        ./testing/perf/dauth_testing/base/ \
        ./testing/perf/dauth_testing/metric_set_2/configs/

for FILE in ./testing/perf/dauth_testing/metric_set_2/configs/*
do
  NAME="$(basename $FILE .yaml)"

  for THRESHOLD in 2 4 8
  do
    for NUM_UES in 1 5 10 20 50
    do
      case $THRESHOLD in
        2)
          # Always allow
          ;;

        4)
          # Don't allow with only 2 backups
          if [[ "$FILE" == *"nbu2"* ]]; then
            break
          fi
          ;;

        8)
          # Don't allow unless 8 backups
          if [[ ! "$FILE" == *"nbu8"* ]]; then
            break
          fi
          ;;
      esac

      echo "Running $NAME with $NUM_UES UE(s) and $THRESHOLD threshold"
      python3 ./testing/perf/run_perf.py \
        -p "$FILE" \
        -c ./testing/configs \
        -u ./testing/perf/ue_driver.py \
        -n "$NUM_UES" \
        -i 2000 \
        -t 10 \
        -k "$THRESHOLD" \
        --backup-auth \
        --debug \
        >> ./testing/perf/dauth_testing/metric_set_2/results/"$NAME".out
    done
  done
done
