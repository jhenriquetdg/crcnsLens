pub mod details;

pub use details::details;

pub struct Collapsing {
    pub describer: String,
    pub is_open: bool,
}

impl Default for Collapsing {
    fn default() -> Self {
        Self {
            describer: String::from("Default"),
            is_open: false,
        }
    }
}
