use std::{collections::HashMap, ops::Deref};

use url::Url;
use vercel_runtime::Request;

pub fn parse_url(req: &Request) -> Result<Url, url::ParseError> {
    Url::parse(&req.uri().to_string())
}

pub fn get_query_param(req: &Request, key: &str) -> Result<Option<String>, url::ParseError> {
    let url = parse_url(req)?;
    let query_map: HashMap<_, _> = url.query_pairs().collect();
    let v = query_map.get(key).map(|v| v.deref().to_owned());
    Ok(v)
}
