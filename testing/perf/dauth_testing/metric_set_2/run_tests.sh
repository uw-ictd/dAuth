ARCHIVE_DIR = $(date --iso-8601=s)
mkdir -p ./testing/perf/dauth_testing/metric_set_2/archive/${ARCHIVE_DIR} 2>/dev/null

mv ./testing/perf/dauth_testing/metric_set_2/configs ./testing/perf/dauth_testing/metric_set_2/archive/${ARCHIVE_DIR}
mv ./testing/perf/dauth_testing/metric_set_2/results ./testing/perf/dauth_testing/metric_set_2/archive/${ARCHIVE_DIR}

mkdir ./testing/perf/dauth_testing/metric_set_2/configs/
mkdir ./testing/perf/dauth_testing/metric_set_2/results/

mkdir ./testing/perf/dauth_testing/metric_set_2/logs/ 2>/dev/null
rm ./testing/perf/dauth_testing/metric_set_2/logs/* 2>/dev/null

python3 ./testing/perf/dauth_testing/metric_set_2/build_configs.py \
        ./testing/perf/dauth_testing/base/ \
        ./testing/perf/dauth_testing/metric_set_2/configs/

for FILE in ./testing/perf/dauth_testing/metric_set_2/configs/*
do
  NAME="$(basename $FILE .yaml)"

  for THRESHOLD in 8
  do
    for NUM_UES in 10 20 40 60 80 100 120 140
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
        -i 30000 \
        -t 0 \
        -k "$THRESHOLD" \
        --backup-auth \
        --debug \
        >> ./testing/perf/dauth_testing/metric_set_2/results/"$NAME".out \
        2>> ./testing/perf/dauth_testing/metric_set_2/logs/"$NAME".log
    done
  done
done
