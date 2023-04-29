use vercel_runtime::{http::ok, *};
use vercel_utils::get_auth_claims_from_cookies;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let claims = get_auth_claims_from_cookies!(&req);
    ok(claims)
}
