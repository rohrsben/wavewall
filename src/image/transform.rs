#[derive(Debug, Clone, Copy)]
pub enum Transform {
    TurnOnce,
    TurnTwice,
    TurnThrice,
    Horizontal,
    Vertical,
    Diagonal,
    Antidiagonal
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Transform::TurnOnce => write!(f, "90"),
            Transform::TurnTwice => write!(f, "180"),
            Transform::TurnThrice => write!(f, "270"),
            Transform::Horizontal => write!(f, "horizontal"),
            Transform::Vertical => write!(f, "vertical"),
            Transform::Diagonal => write!(f, "diagonal"),
            Transform::Antidiagonal => write!(f, "antidiagonal"),
        }
    }
}
