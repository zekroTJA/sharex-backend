use controller::{errs::ErrorKind, Controller};
use sha1::{Digest, Sha1};
use vercel_runtime::{
    http::{bad_request, internal_server_error, not_found},
    *,
};
use vercel_utils::{expect, get_path_param, get_query_param, method_handlers};

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
    let id = get_path_param!(&req, "id");

    let controller = expect!(Controller::from_env().await,
        Err(err) => bad_request(format!("creating controller failed: {err}")));

    let (data, content_type) = expect!(
        controller.get_image(&id).await,
        Err(err) => match err.kind() {
            ErrorKind::ImageNotFound => not_found("not found"),
            _ => internal_server_error(format!("failed getting image from storage: {err}")),
        }
    );

    let mut res = Response::builder().status(StatusCode::OK);
    if let Some(content_type) = content_type {
        res = res.header("Content-Type", content_type);
    }

    let mut hasher = Sha1::new();
    hasher.update(&data);
    let hash = hex::encode(hasher.finalize());
    res = res
        .header("ETag", hash)
        .header("Cache-Control", "public, max-age=30758400");

    Ok(res.body(Body::Binary(data))?)
}
