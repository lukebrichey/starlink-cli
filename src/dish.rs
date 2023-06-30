use async_stream::stream;
use tokio::time::{interval, Duration};

use starlink::proto::space_x::api::device::{
    device_client::DeviceClient, 
    from_device::Message,
    request, 
    GetStatusRequest, 
    Request, 
    Response,
    response,
    to_device,
    ToDevice,
};

pub async fn make_request() -> Result<tonic::Response<Response>, Box<dyn std::error::Error>> {
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

// Creates a stream to continuously request status from the Dish
pub async fn calculate_average_speed() -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let mut client = DeviceClient::connect("http://192.168.100.1:9200").await?;

    let mut interval = interval(Duration::from_millis(100));

    let request_stream = stream! {
        for _ in 0..(15 * 10) {
            yield ToDevice {
                message: Some(to_device::Message::Request(Request {
                    id: None,
                    epoch_id: None,
                    target_id: None,
                    request: Some(request::Request::GetStatus(GetStatusRequest {})),
                })),
            };

            interval.tick().await;
        }
    };

    let request = tonic::Request::new(request_stream);

    let mut response_stream = client.stream(request).await?.into_inner();

    let mut down_speeds: Vec<f64> = Vec::new();
    let mut up_speeds: Vec<f64> = Vec::new();

    while let Some(message) = response_stream.message().await? {
        if let Some(msg) = message.message {
            if let Message::Response(response) = msg {
                if let Some(response::Response::DishGetStatus(response)) = response.response {
                    // Throw out outliers for more accurate results
                    if let Some(downlink_throughput) = response.downlink_throughput_bps {
                        let d_tput = downlink_throughput as f64 / 125_000.0;

                        if d_tput > 10.0 {
                            down_speeds.push(d_tput);
                        }
                    }
                    if let Some(uplink_throughput) = response.uplink_throughput_bps {
                        let u_tput = uplink_throughput as f64 / 125_000.0;

                        if u_tput > 1.0 {
                            up_speeds.push(u_tput);
                        }
                    }
                }
            }
        }
    }

    let avg_down_speed = down_speeds.iter().sum::<f64>() / down_speeds.len() as f64;
    let avg_up_speed = up_speeds.iter().sum::<f64>() / up_speeds.len() as f64;

    Ok((avg_down_speed, avg_up_speed))
}
