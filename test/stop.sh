#!/bin/bash
set -uex

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" > /dev/null 2>&1 && pwd)"
REPO_DIR=$DIR/".."

function kill_tasks() {
  # kill last time running tasks:
  ps aux | grep 'prover-cluster' | grep -v grep | awk '{print $2 " " $11}'
  kill -9 $(ps aux | grep 'prover-cluster' | grep -v grep | awk '{print $2}') || true
}

function stop_docker_compose() {
  dir=$1
  name=$2
  docker-compose --file $dir/docker/docker-compose.yaml --project-name $name down --remove-orphans
  sudo rm -rf $dir/docker/data
  sudo rm -rf $dir/docker/volumes
}

if [[ -z ${AS_RESOURCE+x}  ]]; then
  kill_tasks
  stop_docker_compose $REPO_DIR prover_cluster
fi
