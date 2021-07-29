tonic::include_proto!("cluster");

use fluidex_common::db::models::task::CircuitType;

impl From<Circuit> for CircuitType {
    fn from(pb_circuit: Circuit) -> Self {
        match pb_circuit {
            Circuit::Block => Self::BLOCK,
        }
    }
}
