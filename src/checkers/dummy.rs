use crate::checkers;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Checker {}

impl checkers::Checker for Checker {
    fn check(&self, host: &str) -> Result<(), String> {
        Ok(())
    }
}
