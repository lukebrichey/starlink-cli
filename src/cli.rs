use crate::dish::calculate_average_speed;
use crate::helpers::{print_color_percentage, print_colored};
use starlink::proto::space_x::api::device::{response, Response};
use std::io::{self, Write};
use structopt::StructOpt;
use termion::color;
use tokio::sync::oneshot;
use tokio::time::Duration;

#[derive(StructOpt, Debug)]
pub enum CliOption {
    /// Display alerts
    Alerts,
    /// Display device info
    Info,
    /// Display obstruction stats
    Obstruction,
    /// Display average speed
    Speed,
    /// Display device state
    State,
}


#[derive(StructOpt, Debug)]
#[structopt(
    name = "starlink-cli",
    about = "A CLI tool to interact with your Starlink dish."
)]
pub struct Opt {
    /// The option chosen by the user
    #[structopt(subcommand)]
    pub option: CliOption,
}

// Matches subcommand with response and prints the result
pub fn print_option(option: CliOption, get_status_res: &Response) {
    if let Some(response::Response::DishGetStatus(response)) = &get_status_res.response {
        match option {
            CliOption::Alerts => {
                if let Some(alerts) = &response.alerts {
                    print_colored("Motors stuck: ", &alerts.motors_stuck);
                    print_colored("thermal_throttle: ", &alerts.thermal_throttle);
                    print_colored("thermal_shutdown: ", &alerts.thermal_shutdown);
                    print_colored("mast_not_near_vertical: ", &alerts.mast_not_near_vertical);
                    print_colored("unexpected_location: ", &alerts.unexpected_location);
                    print_colored("slow_ethernet_speeds: ", &alerts.slow_ethernet_speeds);
                }
            }
            CliOption::Info => {
                if let Some(device_info) = &response.device_info {
                    if let Some(id) = &device_info.id {
                        println!("Device id: {}", id)
                    }
                    if let Some(hardware_version) = &device_info.hardware_version {
                        println!("Hardware version: {}", hardware_version)
                    }
                    if let Some(software_version) = &device_info.software_version {
                        println!("Software version: {}", software_version)                
                    }
                    if let Some(country_code) = &device_info.country_code {
                        println!("Country code: {}", country_code)
                    }
                }
            }
            CliOption::Obstruction => {
                if let Some(obstructions_stats) = &response.obstruction_stats {
                    print_colored(
                        "Currently obstructed",
                        &obstructions_stats.currently_obstructed,
                    );
                    match &obstructions_stats.fraction_obstructed {
                        Some(percentage) => {
                            print!("Percentage obstructed: ");
                            print_color_percentage(percentage * 100.0)
                        }
                        None => print_colored::<f32>("Percentage obstructed: ", &None),
                    }
                }
            }
            CliOption::State => {
                if let Some(device_state) = &response.device_state {
                    match device_state.uptime_s {
                        Some(uptime) => {
                            let uptime = uptime as u64;

                            let months = uptime / (30 * 24 * 60 * 60);
                            let weeks = (uptime % (30 * 24 * 60 * 60)) / (7 * 24 * 60 * 60);
                            let days = (uptime % (7 * 24 * 60 * 60)) / (24 * 60 * 60);
                            let seconds = uptime % (24 * 60 * 60);
                            println!(
                                "Uptime: {} months, {} weeks, {} days, {} hours, {} minutes, {} seconds",
                                months, weeks, days, seconds / 3600, (seconds / 60) % 60, seconds % 60
                            );
                        },
                        None => print_colored::<u64>("Uptime: ", &None),
                    }

                }
            }
            _ => {}
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
    println!(
        "\nAverage download speed: {}{:.2}{} Mbps",
        color::Fg(color::Green),
        avg_down,
        color::Fg(color::Reset)
    );
    println!(
        "Average upload speed: {}{:.2}{} Mbps",
        color::Fg(color::Green),
        avg_up,
        color::Fg(color::Reset)
    );
}
