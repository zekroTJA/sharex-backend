#[macro_export]
macro_rules! expect {
    ($expression:expr, $bail:expr) => {
        match $expression {
            Some(v) => v,
            None => {
                return $bail;
            }
        }
    };

    ($expression:expr, $pattern:pat_param => $bail:expr) => {
        match $expression {
            Ok(v) => v,
            $pattern => {
                return $bail;
            }
        }
    };
}

#[macro_export]
macro_rules! get_path_param {
    ($req:expr, $key:expr) => {{
        let v = expect!(get_query_param($req, $key),
            Err(err) => vercel_runtime::http::internal_server_error(
                format!("failed parsing url: {err}")));
        expect!(v, internal_server_error(format!(
            "query params does not contain a value for {} - this should never happen", $key)))
    }};
}
