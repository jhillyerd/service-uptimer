pub mod dummy;
pub mod tcp;

pub trait Checker {
    /// Perform configured check against host, returning Ok or Err(String) describing the failure.
    fn check(&self, host: &str) -> Result<(), String>;
}
