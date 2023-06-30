use starlink::proto::space_x::api::device::{device_client::DeviceClient, request, GetStatusRequest, Request, Response};

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
