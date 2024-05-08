
#[derive(Debug, Clone)]
pub enum Size {
    Fixed(i32),
    Expand(i32),
}

impl Size {
    pub fn without_expand(&self) -> i32 {
        match self {
            Self::Expand(_) => 0,
            Self::Fixed(n) => n.to_owned(),
        }
    }

    pub fn is_expand(&self) -> bool {
        match self {
            Self::Expand(_) => true,
            Self::Fixed(_) => false,
        }
    }

    pub fn fixed_or(&self, default: i32) -> i32 {
        match self {
            Self::Expand(_) => default,
            Self::Fixed(n) => *n,
        }
    }

    pub fn as_fixed(self) -> i32 {
        match self {
            Self::Expand(n) => n,
            Self::Fixed(n) => n,
        }
    }
}