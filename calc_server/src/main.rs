use std::borrow::Borrow;

use futures::{sink::SinkExt, StreamExt};
use nanoservices_utils::{
    register_contract_routes,
    errors::{NanoServiceError, NanoServiceErrorStatus},
    networking::codec::BincodeCodec
};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;

use kernel::{
    TestContractHandler,
    CalcWork,
    WorkType
};


async fn handle_contract_work(mut contract: CalcWork) -> Result<CalcWork, NanoServiceError> {
    let data1 = contract.input_data1;
    let data2 = contract.input_data2;
    if data1 < 0 || data2 < 0 {
        contract.error = Some(NanoServiceError::new(
            "Input data must be a positive integer".to_string(),
            NanoServiceErrorStatus::BadRequest)
        );
    } else {
        contract.result = match &contract.work_type.clone() {
            WorkType::Sum => Some(data1 + data2),
            WorkType::Diff   =>  Some(data1 - data2),
            WorkType::Mult  => Some(data1 * data2),
            WorkType::Div   => Some(data1 / data2),
            _ => None
        };
    }
    Ok(contract)
}



register_contract_routes!(
    TestContractHandler,                  // Struct handling contract serialization
    handle_contract_routes,               // Generate an overall contract handler function of this name
    CalcWork => handle_contract_work   // Map a contract struct to existing handler function
);


#[tokio::main]
async fn main() {
    let bind_server = "127.0.0.1:8080";
    let listener = TcpListener::bind(bind_server).await.unwrap();
    println!("Server listening on port 8080");

    while let Ok((socket, _)) = listener.accept().await {
        let mut framed = Framed::new(socket, BincodeCodec::<TestContractHandler>::new());

        while let Some(result) = framed.next().await {
            match result {
                Ok(data) => {
                    println!("Received: {:?}", data);
                    let response = handle_contract_routes(data).await.unwrap();
                    framed.send(response).await.unwrap();
                    break;
                },
                Err(e) => {
                    eprintln!("Error processing data: {}", e);
                    break;
                }
            }
        }
    }
}