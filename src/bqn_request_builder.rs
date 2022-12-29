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
            out += &(entry.0.to_string() + "⇐" + &entry.1.stringify());
        }

        out += "}";

        out
    }

    pub fn stringify(&self) -> String {
        match self {
            BQNObject::String(string) => "\"".to_string() + string + "\"",
            BQNObject::Number(number) => number.to_string(),
            BQNObject::Array(array) => Self::stringify_array(array),
            BQNObject::Namespace(namespace) => Self::stringify_namespace(namespace),
        }
    }
}

pub fn build_bqn_request(req: Request<Incoming>) -> BQNValue {
    let object = BQNObject::Namespace(vec![
        ("method", BQNObject::String(req.method().to_string()))
    ]).stringify();

    eval(&object)
}
