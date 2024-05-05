
#[derive(Debug, Clone)]
pub enum ContainerDirection {
    Horizontal,
    Vertical,
}

impl From<Option<String>> for ContainerDirection {
    fn from(value: Option<String>) -> Self {
        match value.as_deref() {
            Some("vertical") => Self::Vertical,
            Some("horizontal") => Self::Horizontal,
            _ => Self::Horizontal,
        }
    }
}