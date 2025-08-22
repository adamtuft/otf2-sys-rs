#[cfg(feature = "serde")]
pub trait AsJson: serde::Serialize {
    fn as_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    fn as_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(feature = "serde")]
impl<T> AsJson for T where T: serde::Serialize {}

#[cfg(not(feature = "serde"))]
pub trait AsJson {}
