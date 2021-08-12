#!/bin/bash

# example usage: ./deploy-coordinator.sh 2 2 7 2 50055 postgres://coordinator:coordinator_AA9944@127.0.0.1/prover_cluster

N_TXS=${1:-2}
BALANCE_LEVELS=${2:-2}
ORDER_LEVELS=${3:-7}
ACCOUNT_LEVELS=${4:-2}

PORT=${5:-50055}
DB_URL=${6:-postgres://coordinator:coordinator_AA9944@127.0.0.1/prover_cluster}

apt-get update
apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    ssh \
    libssl-dev \
    apt-utils \
    pkg-config \
    python \
    libgmp-dev \
    nasm \
    nlohmann-json3-dev
apt-get clean
rm -rf /var/lib/apt/lists/*
apt-get update

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
source $HOME/.cargo/env

# echo '
# [source.crates-io]
# registry = "https://github.com/rust-lang/crates.io-index"
# replace-with = "tuna"
# [source.tuna]
# registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
# ' >> $HOME/.cargo/config

# install snarkit
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | sh
source $HOME/.bashrc
source $HOME/.bash_profile
source $HOME/.profile
source /root/.bashrc
source /root/.profile
nvm install --lts
nvm use --lts
npm -g install snarkit

mkdir -p $HOME/repos
git clone https://github.com/fluidex/circuits.git $HOME/repos/fluidex/circuits
git clone https://github.com/fluidex/prover-cluster.git $HOME/repos/fluidex/prover-cluster

cd $HOME/repos/fluidex/circuits
npm install
cp src block -r
mv block/block.circom block/circuit.circom
printf '
component main = Block(%d, %d, %d, %d);
' $N_TXS $BALANCE_LEVELS $ORDER_LEVELS $ACCOUNT_LEVELS >> block/circuit.circom
snarkit compile block

cd $HOME/repos/fluidex/prover-cluster
cargo build --release
printf '
port: %d
db: "%s"
witgen:
  interval: 10000
  n_workers: 5
  circuits:
    block: "%s/repos/fluidex/circuits/block/circuit"
' $PORT $DB_URL $HOME > $HOME/repos/fluidex/prover-cluster/config/coordinator.yaml

# $HOME/repos/fluidex/prover-cluster/target/release/coordinator
nohup $HOME/repos/fluidex/prover-cluster/target/release/coordinator >> $HOME/repos/fluidex/prover-cluster/log-coordinator.txt 2>&1 &
