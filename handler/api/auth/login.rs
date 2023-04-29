use oidc_auth::Exchanger;
use vercel_runtime::{http::internal_server_error, *};
use vercel_utils::expect;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let client = expect!(Exchanger::from_env(), 
        Err(err) => internal_server_error(format!("failed initializing auth excahnger: {err}")));
    let location = client.get_redirect_url(&["openid", "email", "profile"]);

    Ok(Response::builder()
        .header("Location", location)
        .status(StatusCode::FOUND)
        .body(Body::Empty)?)
}
