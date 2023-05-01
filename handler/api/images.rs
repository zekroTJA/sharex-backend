use controller::Controller;
use multipart::server::Multipart;
use std::io::Read;
use vercel_runtime::{
    http::{bad_request, internal_server_error, ok},
    *,
};
use vercel_utils::{expect, get_auth_claims_from_cookies, method_handlers};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    method_handlers!(req,
        "POST" => handler_post(req).await,
    )
}

pub async fn handler_post(req: Request) -> Result<Response<Body>, Error> {
    let claims = get_auth_claims_from_cookies!(&req);

    let boundary = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("multipart/form-data; boundary="));

    let boundary = expect!(boundary, bad_request("invalid content type header"));

    let body = req.body().to_vec();
    let mut data = Multipart::with_body(body.as_slice(), boundary);

    let image = expect!(data.read_entry(), 
        Err(err) => bad_request(format!("failed parsing body: {err}")));

    let mut image = expect!(image, bad_request("body does not contain any image"));

    let content_type = expect!(
        image.headers.content_type.map(|v| v.to_string()),
        bad_request("formdata value does not contain content type")
    );

    let mut image_data: Vec<u8> = vec![];
    expect!(image.data.read_to_end(&mut image_data),
        Err(err) => internal_server_error(format!("could not read body data: {err}")));

    let controller = expect!(Controller::from_env().await,
        Err(err) => bad_request(format!("creating controller failed: {err}")));

    let img = expect!(controller.upload_image(&claims.sub, &image_data, &content_type).await,
        Err(err) => internal_server_error(err.to_string()));

    ok(img)
}
