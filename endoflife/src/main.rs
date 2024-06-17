use endoflife::request::api_request_all_rust_cycles;
use endoflife::rust::RustSingleCycle;

fn main() {
    let _all_cycles: Vec<RustSingleCycle> = api_request_all_rust_cycles().unwrap();
}
