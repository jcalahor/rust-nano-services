use futures::{sink::SinkExt, StreamExt};
use nanoservices_utils::{
    create_contract_handler,
    errors::{NanoServiceError, NanoServiceErrorStatus},
    networking::codec::BincodeCodec
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum WorkType {
    Sum,
    Diff,
    Div,
    Mult
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CalcWork {
    pub input_data1: i32,
    pub input_data2: i32,
    pub work_type: WorkType,
    pub result: Option<i32>,
    pub error: Option<NanoServiceError>,
}



create_contract_handler!(
    TestContractHandler, // this handler struct is created by the macro
    CalcWork
);