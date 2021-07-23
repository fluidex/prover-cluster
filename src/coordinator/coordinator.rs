use crate::coordinator::db::{ConnectionType, MIGRATOR};
use crate::coordinator::{Controller, Settings, WitnessFactory};
use crate::pb::cluster_server::Cluster;
use crate::pb::*;
use sqlx::Connection;
use std::fmt::Debug;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};
use tonic::{Request, Response, Status};

// TODO: auto clean too old entries

type StubType = Arc<RwLock<Controller>>;
type ControllerAction = Box<dyn FnOnce(StubType) -> Pin<Box<dyn futures::Future<Output = ()> + Send>> + Send>;
struct ControllerDispatch<OT>(ControllerAction, oneshot::Receiver<OT>);
impl<OT: 'static /*+ Debug*/ + Send> ControllerDispatch<OT> {
    fn new<T>(f: T) -> Self
    where
        T: for<'c> FnOnce(&'c mut Controller) -> Pin<Box<dyn futures::Future<Output = OT> + Send + 'c>>,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        ControllerDispatch(
            Box::new(
                move |ctrl: StubType| -> Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>> {
                    Box::pin(async move {
                        let mut wg = ctrl.write().await;
                        if let Err(_t) = tx.send(f(&mut wg).await) {
                            // log::error!("Controller action can not be return: {:?}", t);
                            log::error!("Controller action can not be return");
                        }
                    })
                },
            ),
            rx,
        )
    }
}

fn map_dispatch_err<T: 'static>(_: mpsc::error::SendError<T>) -> tonic::Status {
    tonic::Status::unknown("Server temporary unavaliable")
}

type ControllerRet<OT> = Result<OT, tonic::Status>;
type ServerRet<OT> = Result<Response<OT>, tonic::Status>;

fn map_dispatch_ret<OT: 'static>(recv_ret: Result<ControllerRet<OT>, oneshot::error::RecvError>) -> ServerRet<OT> {
    match recv_ret {
        Ok(ret) => ret.map(Response::new),
        Err(_) => Err(Status::unknown("Dispatch ret unreach")),
    }
}

pub struct ServerLeave(mpsc::Sender<ControllerAction>, oneshot::Sender<()>);
impl ServerLeave {
    pub async fn leave(self) {
        self.1.send(()).unwrap();
        self.0.closed().await;
    }
}

#[derive(Debug)]
pub struct Coordinator {
    controller: StubType,
    task_dispacther: mpsc::Sender<ControllerAction>,
    set_close: Option<oneshot::Sender<()>>,
}

impl Coordinator {
    pub async fn from_config(config: &Settings) -> anyhow::Result<Self> {
        let mut db_conn = ConnectionType::connect(&config.db).await?;
        MIGRATOR.run(&mut db_conn).await?;

        let witness_factory = WitnessFactory::from_config(config).await?;
        tokio::spawn(witness_factory.run());

        let controller = Controller::from_config(config).await?;
        let stub = Arc::new(RwLock::new(controller));

        //we always wait so the size of channel doesn't matter
        let (tx, mut rx) = mpsc::channel(16);
        let (tx_close, mut rx_close) = oneshot::channel();

        let stub_for_dispatch = stub.clone();

        let ret = Self {
            task_dispacther: tx,
            set_close: Some(tx_close),
            controller: stub,
        };

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    may_task = rx.recv() => {
                        let task = may_task.expect("Server scheduler has unexpected exit");
                        task(stub_for_dispatch.clone()).await;
                    }
                    _ = &mut rx_close => {
                        log::info!("Server scheduler is notified to close");
                        rx.close();
                        break;
                    }
                }
            }

            //drain unhandled task
            while let Some(task) = rx.recv().await {
                task(stub_for_dispatch.clone()).await;
            }

            log::warn!("Server scheduler has exited");
        });

        Ok(ret)
    }

    pub fn on_leave(&mut self) -> ServerLeave {
        ServerLeave(
            self.task_dispacther.clone(),
            self.set_close.take().expect("Do not call twice with on_leave"),
        )
    }
}

#[tonic::async_trait]
impl Cluster for Coordinator {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        let ControllerDispatch(act, rt) =
            ControllerDispatch::new(move |ctrl: &mut Controller| Box::pin(ctrl.register(request.into_inner())));
        self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
        map_dispatch_ret(rt.await)
    }

    async fn poll_task(&self, request: Request<PollTaskRequest>) -> Result<Response<Task>, Status> {
        let ControllerDispatch(act, rt) =
            ControllerDispatch::new(move |ctrl: &mut Controller| Box::pin(ctrl.poll_task(request.into_inner())));
        self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
        map_dispatch_ret(rt.await)
    }

    async fn submit_proof(&self, request: Request<SubmitProofRequest>) -> Result<Response<SubmitProofResponse>, Status> {
        let ControllerDispatch(act, rt) =
            ControllerDispatch::new(move |ctrl: &mut Controller| Box::pin(ctrl.submit_proof(request.into_inner())));
        self.task_dispacther.send(act).await.map_err(map_dispatch_err)?;
        map_dispatch_ret(rt.await)
    }
}
