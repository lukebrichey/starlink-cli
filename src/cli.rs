use std::io::{self, Write};
use tokio::sync::oneshot;
use tokio::time::Duration;
use structopt::StructOpt;
use termion::color;
use crate::dish::calculate_average_speed;
use crate::helpers::{print_colored, print_color_percentage};
use starlink::proto::space_x::api::device::{response, Response};

#[derive(StructOpt, Debug)]
#[structopt(name = "starlink-cli", about = "A CLI tool to interface with your Starlink dish.")]
pub struct Opt {
    /// Returns the alerts of dish
    #[structopt(short, long)]
    pub alerts: bool,

    /// Returns the state of dish
    #[structopt(long)]
    pub state: bool,

    /// Returns download and upload speed in mbps
    #[structopt(short, long)]
    pub speed: bool,

    #[structopt(short, long)]
    pub obstruction: bool,
}

pub fn print_alerts(get_status_res: &Response) {
    if let Some(response::Response::DishGetStatus(response)) = &get_status_res.response {
        if let Some(alerts) = &response.alerts {
            print_colored("Motors stuck: ", &alerts.motors_stuck);
            print_colored("thermal_throttle: ", &alerts.thermal_throttle);
            print_colored("thermal_shutdown: ", &alerts.thermal_shutdown);
            print_colored("mast_not_near_vertical: ", &alerts.mast_not_near_vertical);
            print_colored("unexpected_location: ", &alerts.unexpected_location);
            print_colored("slow_ethernet_speeds: ", &alerts.slow_ethernet_speeds);
        }
    }
}

// Calculates average speed by throwing out outliers, and then averaging the rest
pub async fn print_speeds() {
    let (tx, rx) = oneshot::channel();
    let mut rx = rx; // make rx mutable

    let calculation_task = tokio::task::spawn(async move {
        let (avg_down, avg_up) = calculate_average_speed().await.unwrap();
        let _ = tx.send(());
        (avg_down, avg_up)
    });

    let mut stdout = io::stdout();
    let mut dots = String::new();

    loop {
        if let Ok(_) = rx.try_recv() {
            break;
        }
        dots.push('.');
        if dots.len() > 3 {
            dots.clear();
        }
        print!("\r\x1B[KCalculating average speeds{}", dots);
        stdout.flush().unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    let (avg_down, avg_up) = calculation_task.await.unwrap();
    println!("\nAverage download speed: {}{:.2}{} Mbps", color::Fg(color::Green), avg_down, color::Fg(color::Reset));
    println!("Average upload speed: {}{:.2}{} Mbps", color::Fg(color::Green), avg_up, color::Fg(color::Reset));
}

// Prints obstruction percentage, colors based on advice from 
// https://www.starlinkhardware.com/starlink-obstructions-how-much-is-too-much/
pub fn print_obstruction(get_status_res: &Response) {
    if let Some(response::Response::DishGetStatus(response)) = &get_status_res.response {
        if let Some(obstructions_stats) = &response.obstruction_stats {
            print_colored("Currently obstructed", &obstructions_stats.currently_obstructed);
            match &obstructions_stats.fraction_obstructed {
                Some(percentage) => { 
                    print!("Percentage obstructed: "); 
                    print_color_percentage(percentage * 100.0)
                },
                None => print_colored::<f32>("Percentage obstructed: ", &None)
            }
        }
    }
}