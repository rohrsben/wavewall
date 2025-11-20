use crate::AppError;

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

impl std::str::FromStr for Transform {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // REMINDER: if you change these, change them in create_pseudos() also
            //   or, figure out a better way to make create_pseudos() lol
            "90" => Ok(Transform::TurnOnce),
            "180" => Ok(Transform::TurnTwice),
            "270" => Ok(Transform::TurnThrice),
            "horizontal" => Ok(Transform::Horizontal),
            "vertical" => Ok(Transform::Vertical),
            "diagonal" => Ok(Transform::Diagonal),
            "antidiagonal" => Ok(Transform::Antidiagonal),
            _ => Err(AppError::TransformParse)
        }
    }
}
