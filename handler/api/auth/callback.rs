use oidc_auth::Exchanger;
use std::env;
use url::Url;
use vercel_runtime::{
    http::{bad_request, internal_server_error},
    *,
};
use vercel_utils::expect;

const MAX_AGE: i64 = 31 * 24 * 3600;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let uri = expect!(Url::parse(&req.uri().to_string()), 
        Err(err) => internal_server_error(format!("failed serializing request uri: {err}")));
    let code = expect!(
        uri.query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, val)| val),
        bad_request("request does not contain an authorization code")
    );
    let client = expect!(Exchanger::from_env(), 
        Err(err) => internal_server_error(format!("failed initializing auth excahnger: {err}")));
    let token = expect!(client.exchange_code_for_token(&code).await, 
        Err(err) => bad_request(format!("token exchange failed: {err}")));
    let id_token = expect!(
        token.id_token,
        bad_request("token exchange response does not contain an id_token")
    );

    let login_redirect = env::var("APP_LOGINREDIRECT").unwrap_or_else(|_| "/".into());
    let vercel_env = env::var("VERCEL_ENV").unwrap_or_else(|_| "development".into());

    let mut cookie = format!("id_token={id_token}; Max-Age={MAX_AGE}; Path=/; HttpOnly");
    if vercel_env != "development" {
        cookie.push_str("; Secure")
    }

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", login_redirect)
        .header("Set-Cookie", cookie)
        .body(Body::Empty)?)
}
