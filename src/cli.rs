use structopt::StructOpt;
use termion::{color, style};
use starlink::proto::space_x::api::device::{response, Response};

#[derive(StructOpt, Debug)]
#[structopt(name = "starlink-cli", about = "A CLI tool to interface with your Starlink dish.")]
pub struct Opt {
    // Command to return the status of dish
    #[structopt(long)]
    pub state: bool,

    // Command to return the alerts of dish
    #[structopt(short, long)]
    pub alerts: bool,
}

pub fn print_alerts(get_status_res: &Response) {
    if let Some(response::Response::DishGetStatus(response)) = &get_status_res.response {
        if let Some(alerts) = &response.alerts {
            println!("Motors stuck: {:?}", alerts.motors_stuck);
            println!("thermal_throttle: {:?}", alerts.thermal_throttle);
            println!("thermal_shutdown: {:?}", alerts.thermal_shutdown);
            println!("mast_not_near_vertical: {:?}", alerts.mast_not_near_vertical);
            println!("unexpected_location: {:?}", alerts.unexpected_location);
            println!("slow_ethernet_speeds: {:?}", alerts.slow_ethernet_speeds);
        }
    }
}
