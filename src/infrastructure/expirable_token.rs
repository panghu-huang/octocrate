pub trait ExpirableToken {
    fn is_expired(&self) -> bool;
    fn get_token(&mut self) -> Option<String>;
}
