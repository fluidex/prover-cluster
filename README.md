# Plonk Prover Cluster

ZK-Rollup is a promising layer 2 technology to help scale Ethereum while a ZK-Rollup system needs a lot of cryptography computations when `proving`. As we estimitated in [FluiDex Labs](https://github.com/fluidex), at least thoudsands of CPU cores are needed to achieve hundreds of TPS. And that is the motivation why we develop this project to work with PLONK proof system.

To generate proofs, two phases are involved: witness generation and proof calculation. A witness is mapped from a given input to the circuit, and a proof is generated according to the witness. The later one is the most computation-heavy part and, as we state, may require 10-100 or more CPU time.

This project supports both witness generation and proving. For witness generation, currently only the `circom` DSL is supported as the first-class circuit language.

# Architecture & Design

Nodes of the prover cluster can be divided into:

1. a single coordinator (or master) node. This node is used to dispatch proving tasks and manage status, by communicating with a database which maintains proving task inputs/witness/proof/status. Proving tasks, along with circuit_types and the associated inputs, are inserted into the database. Once the proving is done, the proof results submitted by prover nodes can be recorded into and queried from the database later.

2. a couple of (stateless) prover nodes. Prover nodes keep requesting tasks from coordinator, calculating the proofs and submitting them to the coordinator.

# Deployment

See [deploy](./deploy/).

# Tech Internals & Dependencies

+ [snarkit](https://github.com/fluidex/snarkit) is used to compile circuit codes to R1CS and create a native binary to produce witnesses.   
+ [plonkit](https://github.com/fluidex/plonkit) is used to generate proving/verification keys.   
+ [bellman_ce](https://github.com/matter-labs/bellman) is used as the internal PLONK proof system lib.   

