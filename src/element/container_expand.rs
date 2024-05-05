

#[derive(Debug, Clone)]
pub enum ContainerExpand {
    All,
    Width,
    Height,
}

impl From<Option<String>> for ContainerExpand {
    fn from(value: Option<String>) -> Self {
        match value.as_deref() {
            Some("width") => Self::Width,
            Some("height") => Self::Height,
            _ => Self::All,
        }
    }
}