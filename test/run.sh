#!/bin/bash
set -uex

# assume already install: libgmp-dev nasm nlohmann-json3-dev snarkit plonkit
# see https://github.com/fluidex/fluidex-backend/blob/master/scripts/install_deps.sh

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" > /dev/null 2>&1 && pwd)"
REPO_DIR=$DIR/".."
PLONKIT_DIR=$REPO_DIR/plonkit
CIRCUIT_DIR=$PLONKIT_DIR/test/circuits/simple

function handle_submodule() {
  git submodule update --init --recursive
  if [ -z ${CI+x} ]; then git pull --recurse-submodules; fi
}


function prepare_circuit() {
  # cd $CIRCUIT_DIR
  # npm i
  snarkit compile $CIRCUIT_DIR --verbose --backend=auto 2>&1 | tee /tmp/snarkit.log
  plonkit dump-lagrange -c $CIRCUIT_DIR/circuit.r1cs -m $PLONKIT_DIR/keys/setup/setup_2^10.key -l $PLONKIT_DIR/keys/setup/setup_2^10.lag.key
}

function prepare_config() {
  cd $DIR
  cp *.yaml $REPO_DIR/config/
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
  # handle_submodule
  prepare_circuit
  prepare_config
}

function run_all() {
  run_docker_compose
}

if [[ -z ${AS_RESOURCE+x}  ]]; then
  setup
  run_all
fi
