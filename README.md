# Plonk Prover Cluster

ZK-Rollup is a promising layer 2 technology to help scale Ethereum. ZK-Rollup system needs a lot of cryptography computation(called `proving`) to support high performance layer 2 transactions, usually at least thoudsands of CPU cores are needed to achieve hundreds of TPS. 

So [we Fluidex Lab](https://github.com/Fluidex/) are developing this proving cluster to work with PLONK proof system.     

The general `proving` can be divided into two phases. First, you generate circuit witnesses from inputs. Then, you generate proof from witnesses. Sometimes we use `prove` to refer only second calculation, Sometimes we use `prove` to refer both calculations. Both types of calculation can be run parallelly for ZK-rollup layer 2 blocks. Usually the second calculation needs 10-100 more CPU time.     

This project supports both witness generation and proving. For witness generation, currently only the `circom` DSL is supported as first class.

# Architecture & Design

Nodes of the proving cluster can be divided into several types: 

1. the single coordinator(or master) node. The node is used to dispatch proving tasks and manage status, by communicating with a database which stores proving task inputs/witness/proof/status. You insert proving task(circuit name  + inputs) into the database and poll for the proof results. 
2. proving nodes. These nodes are stateless. When they are started, they request tasks from coordinator and do the calculation, once the tasks are finished, they submmit the proof to the coordinator and request for new tasks, and so on. 

# Usage Examples

**This repo is under active development, so the docs may not be accurate all the time**

```
# First, write your zk-rollup circuits using the `circom` DSL.
# Assume your circuit codes lie in `block` dir. 
# write your codes here. 
$ vim block/circuit.circom 

# Then compile the circuit DSL file to generate a native executable,
# the executable will be used to generate witnesses from circuit inputs
# After running this command, you will get a 'circuit' binary and 'circuit.r1cs'
npx snarkit compile block 

# Use plonkit to generate circuit keys for this circuit
# you should install plonkit first. See https://github.com/Fluidex/plonkit
# you will get proving SRC keys (monomial key and lagrange key) and verification keys (vk.bin) after this command
cd block
plonkit setup --power 20 --srs_monomial_form mon.key
plonkit dump-lagrange -c circuit.r1cs --srs_monomial_form mon.key --srs_lagrange_form lan.key
plonkit export-verification-key -c circuit.r1cs --srs_monomial_form mon.key

# config coordinator 
# Currently witness generator runs in the same process with coordinator
# in the yaml file, change 'block' to the circuit binary path, update 'db' too
vim config/coordinator.yaml

# launch db and coordinator
cd docker; docker-compose up
cargo build --release
./target/release/coordinator

# config prover client nodes
# every prover instance should have its own config/client.yaml with uniq 'prover_id'
# correct 'circuit' and 'r1cs' and 'src_{monomial,lagrange}_from' should be provided
# 'upstream' is the coordinator address
vim config/client.yaml

# The you use docker-compose / k8s / AWS ECS / Terraform / ansible to lauch all the proving clients
./target/release/coordinator

# Ok it is done. Now you can insert proving task to db then poll for the proof results.
```

# Tech Internals & Dependencies

[snarkit](https://github.com/Fluidex/snarkit) is used to compile circuit codes to R1CS and create a native binary to produce witnesses.   
[plonkit](https://github.com/Fluidex/plonkit) is used to generate circuit keys.   
[bellman_ce](https://github.com/matter-labs/bellman) is used as the PLONK proof system lib.   

