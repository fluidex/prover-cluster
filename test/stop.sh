#!/bin/bash
set -uex

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" > /dev/null 2>&1 && pwd)"
REPO_DIR=$DIR/".."

function stop_docker_compose() {
  dir=$1
  name=$2
  docker-compose --file $dir/docker/docker-compose.yaml --project-name $name down --remove-orphans
  sudo rm -rf $dir/docker/data
  sudo rm -rf $dir/docker/volumes
}

if [[ -z ${AS_RESOURCE+x}  ]]; then
  stop_docker_compose $REPO_DIR prover_cluster
fi
