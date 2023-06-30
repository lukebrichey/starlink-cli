use structopt::StructOpt;
use starlink::proto::space_x::api::device::{device_client::DeviceClient, request, GetStatusRequest, Request, response, Response};

#[derive(StructOpt, Debug)]
#[structopt(name = "starlink-cli", about = "A CLI tool to interface with your Starlink dish.")]
struct Opt {
    // Command to return the status of dish
    #[structopt(long)]
    state: bool,

    // Command to return the alerts of dish
    #[structopt(short, long)]
    alerts: bool,
}

async fn make_request() -> Result<tonic::Response<Response>, Box<dyn std::error::Error>> {
    let mut client = DeviceClient::connect("http://192.168.100.1:9200").await?;

    let request = tonic::Request::new(Request {
        id: None,
        epoch_id: None,
        target_id: None,
        request: Some(request::Request::GetStatus(GetStatusRequest {})),
    });

    let response = client.handle(request).await?;

    Ok(response)
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let res = rt.block_on(make_request());

    println!("Received gRPC response from Dish, parsing...");

    // Handle errors
    if res.is_err() {
        println!("Error: {:?}", res.err());
        return;
    }

    // Take flag
    let opt: Opt = Opt::from_args();

    // Transform response
    let get_status_res = res.unwrap().into_inner();
    
    if let Some(response::Response::DishGetStatus(response)) = &get_status_res.response {
        if opt.alerts {
            if let Some(alerts) = &response.alerts {
                println!("motors_stuck: {:?}", alerts.motors_stuck);
                println!("thermal_throttle: {:?}", alerts.thermal_throttle);
                println!("thermal_shutdown: {:?}", alerts.thermal_shutdown);
                println!("mast_not_near_vertical: {:?}", alerts.mast_not_near_vertical);
                println!("unexpected_location: {:?}", alerts.unexpected_location);
                println!("slow_ethernet_speeds: {:?}", alerts.slow_ethernet_speeds);
            }
        }
    }
    
}
