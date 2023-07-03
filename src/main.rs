mod dish;
mod cli;
mod helpers;

use structopt::StructOpt;
use dish::make_request;
use cli::{CliOption, Opt, print_option};

fn main() {
    let opt: Opt = Opt::from_args();
    let rt = tokio::runtime::Runtime::new().unwrap();

    match opt.option {
        CliOption::Speed => {
            rt.block_on(cli::print_speeds());
        },
        _ => {
            let res = rt.block_on(make_request());

            // Handle errors
            if res.is_err() {
                println!("Error: {:?}", res.err());
                return;
            }

            // Transform response
            let get_status_res = res.unwrap().into_inner();

            print_option(opt.option, &get_status_res);
        }
    }
}
