#[macro_export]
macro_rules! get_auth_claims_from_cookies {
    ($req:expr) => {
        {
            let id_token = $req
                .headers()
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("id_token "))
                .or_else(|| $crate::get_cookie_value($req, "id_token"));

            let id_token = $crate::expect!(
                id_token,
                vercel_runtime::http::unauthorized("unauthorized")
            );
            let validator = $crate::expect!(
                oidc_auth::Validator::from_env(),
                Err(err) => vercel_runtime::http::internal_server_error(format!("could not construct validator from env: {err}"))
            );
            $crate::expect!(
                validator.decode_and_verify_idtoken(id_token).await,
                Err(_) => vercel_runtime::http::unauthorized("unauthorized")
            )
        }
    };
}
