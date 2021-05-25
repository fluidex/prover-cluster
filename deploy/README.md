# Deployment for prover-cluster

This directory include assets for deploying prover-cluster on container-based workset managament (docker-compose, k8s, etc.).

## Overview

Three base images would be used to start deployment:

+ plonkit: built from `images/plonkit.docker`, enable cryptology setup from plonkit
+ prover: built from `images/prover.docker`, enable running coordinator / client
+ setup: built from `images/ssetup.docker`, enable setup circuits, and integrate them under the plonkit toolsets

## For docker-compose

Use docker-compose.yaml to cast a test cluster, you have more options to integrate setup assets with prover binaries. For example, if you have prepared the setup key, you can put the key in directory `./setup/mon.key` and use `images/cluster_client_test.docker` rather than cluster_client_test.docker, which also make an setup key while building the docker image

The default config provide use a simple test circuit to replace the real "block" circuit to benefit checking of the running of whole clusters. To apply the real block circuit, change all "/opt/test" field into "/opt/block" in client.yaml and "/opt/circuit_test" into "/opt/circuit_block" in coordinator.yaml

## For kuberntes

There maybe not be general setup scripts/config for deploying the whole prover-cluster on a specified kubernetes. We have put as many assets as possible in `commonk8s` for reference.

