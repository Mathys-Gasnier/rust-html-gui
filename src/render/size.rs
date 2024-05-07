
pub type Size = (AxisSize, AxisSize);

pub enum AxisSize {
    Fixed(i32),
    Expand(i32),
}

impl AxisSize {
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

    pub fn fixed_or(self, default: i32) -> i32 {
        match self {
            Self::Expand(_) => default,
            Self::Fixed(n) => n,
        }
    }
}