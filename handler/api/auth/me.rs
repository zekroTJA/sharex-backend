use vercel_runtime::{http::ok, *};
use vercel_utils::{get_auth_claims_from_cookies, method_handlers};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    method_handlers!(req,
        "GET" => handler_get(req).await,
    )
}

pub async fn handler_get(req: Request) -> Result<Response<Body>, Error> {
    let claims = get_auth_claims_from_cookies!(&req);
    ok(claims)
}
