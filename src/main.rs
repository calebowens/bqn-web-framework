use std::fs;
use cbqn::{eval};
use cbqn::BQNValue;

fn main() {
    let file = fs::read_to_string("main.bqn").expect("main.bqn entrypoint expected");

    let runtime = eval(&file);

    let result = runtime.call1(&eval("{ method ⇐ \"PUT\" }")).to_bqnvalue_vec();

    // Result is expected to be [int, [[string, string]], string]
    let status_code = result[0].to_f64() as usize;
    let headers = result[1].to_bqnvalue_vec().iter().map(|item| item.to_bqnvalue_vec().iter().map(BQNValue::to_string).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
    let body = result[2].to_string();

    println!("{} {:?} {}", status_code, headers, body);
}
