use async_trait::async_trait;

pub mod images;
pub mod tokens;

use self::{images::Images, tokens::Tokens};

#[async_trait]
pub trait Persistence: Images + Tokens {}
