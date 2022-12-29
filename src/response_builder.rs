use std::str::FromStr;

use cbqn::BQNValue;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::{HeaderValue, HeaderName};
use hyper::{Response, StatusCode};

pub fn build_response(result: Vec<BQNValue>) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
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
