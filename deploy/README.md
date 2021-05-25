# Deployment for prover-cluster

This directory include assets for deploying prover-cluster on container-based workset managament (docker-compose, k8s, etc.).

## For docker-compose

Use docker-compose.yaml to demo a test cluster, you have flexible choices to integrate setup assets with prover binaries. For example, if you have prepared the monomial SRS, you can put it in `./setup/mon.key` and use `images/cluster_client.docker` rather than `cluster_client_test.docker`(which regenerates an monomial SRS while building the docker image).

The default config provide use a simpler test circuit to replace the real "block" circuit, to benefit verify the whole cluster is functioning . To apply the real block circuit, change all "/opt/test" field into "/opt/block" in client.yaml and "/opt/circuit_test" into "/opt/circuit_block" in coordinator.yaml

## For kuberntes

There maybe not be general setup scripts/config for deploying the whole prover-cluster on a specified kubernetes. We have put as many assets as possible in `commonk8s` for reference.

