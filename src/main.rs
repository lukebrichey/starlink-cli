mod dish;
mod cli;

use structopt::StructOpt;
use dish::make_request;
use cli::Opt;

fn main() {
    let opt: Opt = Opt::from_args();
    let rt = tokio::runtime::Runtime::new().unwrap();

    // If speed flag passed, we create a stream
    if opt.speed {
        rt.block_on(cli::print_speeds());
        return;
    }

    let res = rt.block_on(make_request());

    println!("Received gRPC response from Dish, parsing...");

    // Handle errors
    if res.is_err() {
        println!("Error: {:?}", res.err());
        return;
    }

    // Transform response
    let get_status_res = res.unwrap().into_inner();

    if opt.alerts {
        cli::print_alerts(&get_status_res);
    }
}
