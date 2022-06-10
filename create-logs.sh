#!/bin/bash

LOGS_DIR=$1
REPETITIONS=$2
MAX_THREADS=$3

# Compute how many digits there are in these variables
# in order to add leading zeros to the log files and
# in doing so preserve their alphabetical order.
REPETITIONS_LEN=${#REPETITIONS}
MAX_THREADS_LEN=${#MAX_THREADS}

RUNTIMES=("sequential" "sequential-io" "rust-ssp" "rust-ssp-io" "pipeliner" "tokio" "tokio-io" "rayon" "std-threads" "std-threads-io" "better")
APPS=("bzip2" "eye-detector" "image-processing" "micro-bench")

declare -A AVAILABLE_RUNTIMES=(
	["bzip2"]="sequential rust-ssp pipeliner tokio rayon std-threads sequential-io rust-ssp-io tokio-io std-threads-io"
	["eye-detector"]="sequential rust-ssp tokio std-threads better"
	["image-processing"]="sequential rust-ssp pipeliner tokio rayon std-threads"
	["micro-bench"]="sequential rust-ssp pipeliner tokio rayon std-threads"
    ["gif-encoder"]="sequential rust-ssp rayon"
)

for RUNTIME in "${RUNTIMES[@]}"
do
	echo "RUNTIME=$RUNTIME"

	for ((i = 0; i < $REPETITIONS; i++))
	do
		echo "    i=$i"

		REPETITION_OUT=$(printf "%0${REPETITIONS_LEN}d" ${i})

		for ((THREADS = 1; THREADS <= MAX_THREADS; THREADS++))
		do
			echo "        THREADS=$THREADS"

			THREADS_OUT=$(printf "%0${MAX_THREADS_LEN}d" ${THREADS})

			for APP in "${APPS[@]}"
			do
				if [[ "${AVAILABLE_RUNTIMES[$APP]}" != *"${RUNTIME}"* ]]
				then
					continue
				fi

				echo "            APP=$APP"
				cd RustStreamBench/${APP}

				case "$APP" in
					"bzip2")
						WORKLOAD="wiki_data"
						INPUT="inputs/${WORKLOAD}"

						./target/release/${APP} $RUNTIME $THREADS compress $INPUT \
							>> ../../${LOGS_DIR}/${APP}_${WORKLOAD//_/-}-compress_${RUNTIME}_${THREADS_OUT}_${REPETITION_OUT}

						./target/release/${APP} $RUNTIME $THREADS decompress ${INPUT}.bz2 \
							>> ../../${LOGS_DIR}/${APP}_${WORKLOAD//_/-}-decompress_${RUNTIME}_${THREADS_OUT}_${REPETITION_OUT}
						;;
					"eye-detector")
						if [[ "$RUNTIME" = "sequential" ]]
						then
							RUNTIME_INPUT="seq"
						else
							RUNTIME_INPUT=$RUNTIME
						fi

						WORKLOAD="one_face_15s.mp4"
						INPUT="inputs/${WORKLOAD}"

						./target/release/${APP} $RUNTIME_INPUT $THREADS $INPUT \
							>> ../../${LOGS_DIR}/${APP}_${WORKLOAD//_/-}_${RUNTIME}_${THREADS_OUT}_${REPETITION_OUT}
						;;
					"image-processing")
						WORKLOAD="inputs"

						./target/release/${APP} $RUNTIME $THREADS $WORKLOAD \
							>> ../../${LOGS_DIR}/${APP}_${WORKLOAD//_/-}_${RUNTIME}_${THREADS_OUT}_${REPETITION_OUT}
						;;
					"micro-bench")
						MATRIX_SIZE=2048
						ITER1=3000
						ITER2=2000
						WORKLOAD="${MATRIX_SIZE}-${ITER1}-${ITER2}"

						./target/release/${APP} $RUNTIME $MATRIX_SIZE $THREADS $ITER1 $ITER2 \
							>> ../../${LOGS_DIR}/${APP}_${WORKLOAD//_/-}_${RUNTIME}_${THREADS_OUT}_${REPETITION_OUT}
						;;
                    "gif-encoder")
                        # TODO: completar o workload com o nome certinho do vídeo
                        WORKLOAD=".mp4"
						INPUT="inputs/${WORKLOAD}"

                        ./target/release/${APP} $RUNTIME $THREADS $WORKLOAD \
                            >> ../../${LOGS_DIR}/${APP}_${WORKLOAD//_/-}_${RUNTIME}_${THREADS_OUT}_${REPETITION_OUT}
                        ;;
				esac

				cd ../..
			done
		done
	done
done