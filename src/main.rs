use std::fs;
use std::env;
use cbqn::{eval};
use cbqn::BQNValue;
use std::str::FromStr;

use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use tokio::net::TcpListener;
use hyper::header::{HeaderValue, HeaderName};

fn build_response(result: Vec<BQNValue>) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    let status_code = result[0].to_f64() as u16;
    let headers = result[1]
        .to_bqnvalue_vec()
        .iter()
        .map(|item| item.to_bqnvalue_vec().iter().map(BQNValue::to_string).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let body = result[2].to_string();

    let mut response = Response::new(Full::new(Bytes::from(body)));

    *response.status_mut() = StatusCode::from_u16(status_code).unwrap();

    for header in headers {
        response.headers_mut().insert(
            HeaderName::from_str(&header[0]).unwrap(),
            HeaderValue::from_str(&header[1]).unwrap()
        );
    }

    Ok(response)
}

async fn run(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
    let current_directory = env::current_dir().unwrap();
    let current_directory_str = current_directory.to_str().unwrap();

    let runtime = eval(
        &(
            "⟨ \"".to_string() +
            current_directory_str + "/\", ⟨⟩, ⟨⟩⟩ •BQN •FChars \"" +
            current_directory_str + "/main.bqn\""
        )
    );

    let result = runtime.call1(&eval(&("{ method ⇐ \"".to_string() + req.method().as_str() + "\" }"))).to_bqnvalue_vec();

    build_response(result)
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
