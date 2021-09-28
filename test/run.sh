#!/bin/bash
set -uex

# assume already install: libgmp-dev nasm nlohmann-json3-dev snarkit plonkit
# see https://github.com/fluidex/fluidex-backend/blob/master/scripts/install_deps.sh

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" > /dev/null 2>&1 && pwd)"
REPO_DIR=$DIR/".."
PLONKIT_DIR=$REPO_DIR/plonkit
CIRCUIT_DIR=$PLONKIT_DIR/test/circuits/simple

CURRENTDATE=$(date +"%Y-%m-%d")

function handle_submodule() {
  git submodule update --init --recursive
  if [ -z ${CI+x} ]; then git pull --recurse-submodules; fi
}

function prepare_circuit() {
  snarkit compile $CIRCUIT_DIR --verbose --backend=auto 2>&1 | tee /tmp/snarkit.log
  plonkit export-verification-key -c $CIRCUIT_DIR/circuit.r1cs -m $PLONKIT_DIR/keys/setup/setup_2^10.key -v $CIRCUIT_DIR/vk.bin --overwrite
  cd $PLONKIT_DIR; git update-index --assume-unchanged $CIRCUIT_DIR/vk.bin
  plonkit dump-lagrange -c $CIRCUIT_DIR/circuit.r1cs -m $PLONKIT_DIR/keys/setup/setup_2^10.key -l $PLONKIT_DIR/keys/setup/setup_2^10.lag.key --overwrite
}

function prepare_config() {
  printf 'prover_id: 1
upstream: "http://[::1]:50055"
poll_interval: 10000
srs_monomial_form: "%s/keys/setup/setup_2^10.key"
db: postgres://coordinator:coordinator_AA9944@127.0.0.1:5433/prover_cluster
circuit:
  name: "block"
  bin: "%s/test/circuits/simple/circuit.fast"
  r1cs: "%s/test/circuits/simple/circuit.r1cs"
  vk: "%s/test/circuits/simple/vk.bin"
  srs_lagrange_form: "%s/keys/setup/setup_2^10.lag.key"' $PLONKIT_DIR $PLONKIT_DIR $PLONKIT_DIR $PLONKIT_DIR $PLONKIT_DIR > $REPO_DIR/config/client.yaml

  printf 'port: 50055
db: postgres://coordinator:coordinator_AA9944@127.0.0.1:5433/prover_cluster
circuit:
  vk: "%s/test/circuits/simple/vk.bin"' > $REPO_DIR/config/coordinator.yaml
}

function restart_docker_compose() {
  dir=$1
  name=$2
  docker-compose --file $dir/docker/docker-compose.yaml --project-name $name down --remove-orphans
  sudo rm -rf $dir/docker/data
  sudo rm -rf $dir/docker/volumes
  docker-compose --file $dir/docker/docker-compose.yaml --project-name $name up --force-recreate --detach
}

function run_docker_compose() {
  restart_docker_compose $REPO_DIR prover_cluster
}

function setup() {
  handle_submodule
  prepare_circuit
  prepare_config
}

function init_task() {
  PROVER_DB="postgres://coordinator:coordinator_AA9944@127.0.0.1:5433/prover_cluster"
  psql $PROVER_DB -f $DIR/mock_sqls/init.sql
}

function validate_task() {
  PROVER_DB="postgres://coordinator:coordinator_AA9944@127.0.0.1:5433/prover_cluster"
  # Validate if Task ID of `task_1` is returned as proved.
  if psql $PROVER_DB -f $DIR/mock_sqls/validate.sql | grep -q 'task_1'; then
    echo "Task is proved"
  else
    echo "No proved task with ID of task_1 is returned"
    exit 1
  fi
}

function run_bin() {
  cd $REPO_DIR
  cargo build
  nohup $REPO_DIR/target/debug/coordinator > $REPO_DIR/coordinator.$CURRENTDATE.log 2>&1 &
  nohup $REPO_DIR/target/debug/client > $REPO_DIR/client.$CURRENTDATE.log 2>&1 &
}

function run_all() {
  run_docker_compose
  sleep 5
  run_bin
  sleep 5
  init_task
  sleep 15
  validate_task
}

if [[ -z ${AS_RESOURCE+x}  ]]; then
  . $DIR/stop.sh
  setup
  run_all
fi
