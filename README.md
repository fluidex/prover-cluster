# Plonk Prover Cluster

ZK-Rollup is a promising layer 2 technology to help scale Ethereum while a ZK-Rollup system needs a lot of cryptography computations when `proving`. As we estimitated in [Fluidex Labs](https://github.com/Fluidex), at least thoudsands of CPU cores are needed to achieve hundreds of TPS. And that is the motivation why we develop this project to work with PLONK proof system.

To generate proofs, two phases are involved: witness generation and proof calculation. A witness is mapped from a given input to the circuit, and a proof is generated according to the witness. The later one is the most computation-heavy part and, as we state, may require 10-100 or more CPU time.

This project supports both witness generation and proving. For witness generation, currently only the `circom` DSL is supported as the first-class circuit language.

# Architecture & Design

Nodes of the prover cluster can be divided into:

1. a single coordinator (or master) node. This node is used to dispatch proving tasks and manage status, by communicating with a database which maintains proving task inputs/witness/proof/status. Proving tasks, along with circuit_types and the associated inputs, are inserted into the database. Once the proving is done, the proof results submitted by prover nodes can be recorded into and queried from the database later.

2. a couple of (stateless) prover nodes. Prover nodes keep requesting tasks from coordinator, calculating the proofs and submitting them to the coordinator.

# Usage Examples

**This repo is still under active development, so the docs may not be accurate all the time**

```
# First, write your zk-rollup circuits using the `circom` DSL.
# Assume your circuit codes lie in `block` dir. 
# Write your circuit codes here. 
$ vim block/circuit.circom 

# Then compile the circuit DSL file to generate a native executable,
# the executable will be used to generate witnesses from circuit inputs.
# After running this command, you will get a 'circuit' binary and 'circuit.r1cs'
npx snarkit compile block 

# Use plonkit to generate circuit keys for this circuit.
# You need to install plonkit first. See https://github.com/Fluidex/plonkit
# After running these commands, we will get SRS proving keys (monomial key and lagrange key) and verification keys (vk.bin).
cd block
plonkit setup --power 20 --srs_monomial_form mon.key
plonkit dump-lagrange -c circuit.r1cs --srs_monomial_form mon.key --srs_lagrange_form lag.key
plonkit export-verification-key -c circuit.r1cs --srs_monomial_form mon.key

# Configure the coordinator.
# Currently witness generator runs in the same process as coordinator.
# In the yaml file, modify 'block' value to the circuit binary path, and config 'db' URL as well.
vim config/coordinator.yaml

# Launch db and coordinator.
cd docker; docker-compose up
cargo build --release
./target/release/coordinator

# Configure prover client nodes
# Every prover instance should have its own config/client.yaml with unique 'prover_id'
# Configure 'circuit', 'r1cs', 'src_{monomial,lagrange}_from' and 'upstream' (the coordinator address) to their correct values.
vim config/client.yaml

# The we can use docker-compose / k8s / AWS ECS / Terraform / ansible to launch all the prover clients
./target/release/coordinator

# Now we are all set! Prover nodes will be scheduled to generate proofs if new tasks are inserted into the DB. And the proof results can be queried from the DB at the end.
```

# Tech Internals & Dependencies

+ [snarkit](https://github.com/Fluidex/snarkit) is used to compile circuit codes to R1CS and create a native binary to produce witnesses.   
+ [plonkit](https://github.com/Fluidex/plonkit) is used to generate proving/verification keys.   
+ [bellman_ce](https://github.com/matter-labs/bellman) is used as the internal PLONK proof system lib.   

