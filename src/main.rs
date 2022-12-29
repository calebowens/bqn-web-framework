mod response_builder;
mod bqn_runtime;
mod bqn_request_builder;

use std::net::SocketAddr;

use tokio::net::TcpListener;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};

use response_builder::build_response;
use bqn_runtime::build_runtime;
use bqn_request_builder::build_bqn_request;

async fn run(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    let runtime = build_runtime();

    let bqn_result = runtime.call1(&build_bqn_request(req)).to_bqnvalue_vec();

    build_response(bqn_result)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(run))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
