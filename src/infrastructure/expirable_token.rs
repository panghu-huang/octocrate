use std::fmt::Debug;

pub trait ExpirableToken: Debug {
    fn is_expired(&self) -> bool;
    fn get_token(&self) -> Option<String>;
}
