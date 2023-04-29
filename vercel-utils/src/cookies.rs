use vercel_runtime::Request;

pub fn get_cookies(req: &Request) -> Option<Vec<(&str, &str)>> {
    req.headers()
        .get("Cookie")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.split(';').map(|kv| kv.trim()))
        .map(|split| split.filter_map(|kv| kv.split_once('=')).collect())
}

pub fn get_cookie_value<'a>(req: &'a Request, key: &str) -> Option<&'a str> {
    get_cookies(req)?
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}
