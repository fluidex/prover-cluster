use crate::client::{GrpcClient, Prover, Settings, Witness};
use fluidex_common::db::models::{tablenames, task};
use fluidex_common::db::{DbType, PoolOptions};
use futures::{channel::mpsc, StreamExt};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub enum WatchRequest {
    Register,
    PollTask,
}

pub struct Watcher {
    db_pool: sqlx::Pool<DbType>,
    grpc_client: GrpcClient,
    is_busy: AtomicBool,
    prover: Prover,
    witness: Witness,
}

impl Watcher {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let db_pool = PoolOptions::new().connect(&config.db).await?;
        let grpc_client = GrpcClient::from_config(config);
        let is_busy = AtomicBool::new(false);
        let prover = Prover::from_config(config);
        let witness = Witness::from_config(&config).await?;

        Ok(Self {
            db_pool,
            grpc_client,
            is_busy,
            prover,
            witness,
        })
    }

    pub async fn run(&mut self, mut watch_req: mpsc::Receiver<WatchRequest>) {
        while let Some(request) = watch_req.next().await {
            if self.is_busy.load(Ordering::SeqCst) {
                continue;
            }

            match request {
                WatchRequest::Register => {
                    log::debug!("WatchRequest::Register");
                    self.is_busy.store(true, Ordering::SeqCst);
                    if let Err(e) = self.grpc_client.register().await {
                        log::error!("register client error {:?}", e);
                    }
                    self.is_busy.store(false, Ordering::SeqCst);
                }
                WatchRequest::PollTask => {
                    log::debug!("WatchRequest::PollTask");
                    self.is_busy.store(true, Ordering::SeqCst);

                    let (task_id, circuit) = match self.grpc_client.poll_task().await {
                        Ok(t) => (t.id, t.circuit),
                        Err(e) => {
                            log::error!("poll task error {:?}", e);
                            self.is_busy.store(false, Ordering::SeqCst);
                            continue;
                        }
                    };

                    let task = match self.get_task_by_id(&task_id).await {
                        Ok(t) => t,
                        Err(e) => {
                            log::error!("cannot find task({}) error {:?}", task_id, e);
                            self.is_busy.store(false, Ordering::SeqCst);
                            continue;
                        }
                    };

                    let witness = match self.witness.witgen(task).await {
                        Ok(w) => w,
                        Err(e) => {
                            log::error!("witness task({}) error {:?}", task_id, e);
                            self.is_busy.store(false, Ordering::SeqCst);
                            continue;
                        }
                    };

                    match self.prover.prove(circuit, witness).await {
                        Ok(proof) => {
                            let grpc_client = self.grpc_client.clone();
                            tokio::spawn(/*move ||*/ async move {
                                match grpc_client.submit(&task_id, proof).await {
                                    Ok(resp) => {
                                        log::info!("submission for task({:?}) successful", &task_id);
                                        log::info!("task({:?}) submission result valid: {:?}", &task_id, resp.valid);
                                    }
                                    Err(e) => {
                                        log::error!("submit result for task({:?}) error {:?}", &task_id, e);
                                    }
                                };
                            });
                        }
                        Err(e) => log::error!("{:?}", e),
                    }

                    self.is_busy.store(false, Ordering::SeqCst);
                }
            }
        }
    }

    async fn get_task_by_id(&self, task_id: &str) -> Result<task::Task, anyhow::Error> {
        let query = format!("select * from {} where task_id = $1 limit 1", tablenames::TASK);
        let task: task::Task = sqlx::query_as(&query).bind(task_id).fetch_one(&self.db_pool).await?;

        Ok(task)
    }
}
