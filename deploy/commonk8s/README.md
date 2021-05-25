# Common setup assets for deploying on kubernetes

Assets required by deploying prover-cluster on kubernetes. Not like compose, there maybe many options and customizations when ops is facing a provided kubernetes system and try to accomplish his task. Things put here is far from a "one-key" solution and just act as templates and references 

## Notes and considerations before start

### Prepare your images

We need to prepare the coordinator and client images build from `images/cluster_coordinator.docker` and `images/cluster_client.docker` and being avaliable in the image repo. service of your k8s. In this example we provide these images with aliyun's repo service.

### Tagging and tainting nodes

Prover client require dedicated, bold nodes to running on. We have to mark these nodes inside k8s node pools by both tagging and taintings. 

Inside this deployment we tag these nodes with `fluidexnode=compoutation` and tainting them by `dedicated=computation:NoSchedule`

Tainting and tagging can be executed by commands like `kubectl taint nodes <your nodes> dedicated=computation:NoSchedule`

### Deploy database outside kubernetes

It is often not a good practice to delpoy the database in the same k8s cluster. But we still provide a deloyment asset inside `prerequisite` directory, in which we use peristent volume for data persistent. It is also possible to bind the db pod into a single node and use hostpath as data directory.


## Do the deployment

### Apply config assets

The config assets (`0_configmap.yaml` and `0_secrets.yaml`) should be customized and apply first, which provide the setup data for accesing db, coordinator etc.

### Deploy coordinator

Appling `1_coordinator.yaml` and `2_services.yaml` to setup the coordinator instance, and expose its service inside k8s.

### Deploy clients

Appling `3_client.yaml` to setup mutiple client instances.

## Test and verification

After applied all assets onto k8s, we can verify the running of prover-cluster by following setups

1. The logs of coordinator and client

2. Check if all client pods are running on expected nodes, and each dedicated node should have only one client pod

3. Access the database, insert some test data, for example, for 'poseidon' circuit you could insert

```sql

insert into task (task_id, circuit, input) values 
    ('6', 'block', '{ "foo": 13, "bar": 4 }::jsonb'),
    ('7', 'block', '{ "foo": 14, "bar": 4 }::jsonb'),
    ('8', 'block', '{ "foo": 15, "bar": 4 }::jsonb');

```

4. check if the status column in all lines added into database become "proved"

## Setup the cluster for production

The prover-cluster deployed by these assets use the demo circuit and we need to replace it with the real "block" circuit
