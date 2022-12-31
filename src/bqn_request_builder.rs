use hyper::body::Incoming;
use hyper::Request;
use cbqn::{BQNValue, eval};

pub enum BQNObject {
    String(String),
    Number(f64),
    Array(Vec<BQNObject>),
    Namespace(Vec<(&'static str, BQNObject)>)
}

impl BQNObject {
    fn stringify_array(array: &Vec<BQNObject>) -> String {
        let mut out = "⟨".to_string();

        for object in array {
            out += &(object.stringify() + ",");
        }

        out += "⟩"; 

        out
    }

    fn stringify_namespace(namespace: &Vec<(&'static str, BQNObject)>) -> String {
        let mut out = "{".to_string();

        for entry in namespace {
            out += &(entry.0.to_string() + "⇐" + &entry.1.stringify() + "\n");
        }

        out += "}";

        out
    }

    pub fn stringify(&self) -> String {
        match self {
            BQNObject::String(string) => "\"".to_string() + &string.replace("\"", "\"\"") + "\"",
            BQNObject::Number(number) => number.to_string(),
            BQNObject::Array(array) => Self::stringify_array(array),
            BQNObject::Namespace(namespace) => Self::stringify_namespace(namespace),
        }
    }
}

trait BQNAble {
    fn to_bqn_object(&self) -> BQNObject;
}

impl BQNAble for str {
    fn to_bqn_object(&self) -> BQNObject {
        BQNObject::String(self.to_string())
    }
}

impl BQNAble for f64 {
    fn to_bqn_object(&self) -> BQNObject {
        BQNObject::Number(*self)
    }
}

fn build_path(req: &Request<Incoming>) -> BQNObject {
    BQNObject::Namespace(vec![
        // ("scheme", req.uri().scheme_str().unwrap_or("").to_bqn_object()),
        // ("host", req.uri().host().unwrap_or("").to_bqn_object()),
        // ("port", (req.uri().port_u16().unwrap_or(0) as f64).to_bqn_object()),
        ("path", req.uri().path().to_bqn_object()),
        ("query", req.uri().query().unwrap_or("").to_bqn_object())
    ])
}

fn build_headers(req: &Request<Incoming>) -> BQNObject {
    BQNObject::Array(
        req.headers()
            .iter()
            .map(|(header_name, header_value)| BQNObject::Array(vec![
                header_name.to_string().to_bqn_object(),
                header_value.to_str().unwrap_or("").to_bqn_object()
            ]))
            .collect()
    )
}

pub fn build_bqn_request(req: Request<Incoming>) -> BQNValue {
    let object = BQNObject::Namespace(vec![
        ("method", req.method().to_string().to_bqn_object()),
        ("uri", build_path(&req)),
        ("headers", build_headers(&req))
    ]).stringify();

    eval(&object)
}
