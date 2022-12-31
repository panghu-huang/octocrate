pub trait ExpirableToken {
    fn is_expired(&self) -> bool;
    fn get_token(&self) -> Option<String>;
}
