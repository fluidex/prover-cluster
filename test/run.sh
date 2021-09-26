#!/bin/bash
set -uex

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" > /dev/null 2>&1 && pwd)"
PLONKIT_DIR=$DIR/plonkit

function handle_submodule() {
  git submodule update --init --recursive
  if [ -z ${CI+x} ]; then git pull --recurse-submodules; fi
}

function setup() {
  handle_submodule
}

if [[ -z ${AS_RESOURCE+x}  ]]; then
  setup
fi
