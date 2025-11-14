use crate::error::AppError;

pub fn string(input: mlua::Value, location: String) -> Result<Option<String>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::String(str) => Ok(Some(str.to_string_lossy())),
        _ => Err(AppError::ConfigType(location, format!("nil, string"), input.type_name().to_string()))
    }
}

pub fn string_necessary(input: mlua::Value, location: String) -> Result<String, AppError> {
    match input {
        mlua::Value::String(str) => Ok(str.to_string_lossy()),
        _ => Err(AppError::ConfigType(location, format!("string"), input.type_name().to_string()))
    }
}

pub fn uint(input: mlua::Value, location: String) -> Result<Option<usize>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Integer(int) => {
            if int < 0 {
                return Err(AppError::ConfigType(location, format!("nil, positive number"), input.type_name().to_string()))
            }

            Ok(Some(int as usize))
        }
        _ => Err(AppError::ConfigType(location, format!("nil, positive integer"), input.type_name().to_string()))
    }
}

pub fn uint_necessary(input: mlua::Value, location: String) -> Result<usize, AppError> {
    match input {
        mlua::Value::Integer(int) => {
            if int < 0 {
                return Err(AppError::ConfigType(location, format!("positive number"), input.type_name().to_string()))
            }

            Ok(int as usize)
        }
        _ => Err(AppError::ConfigType(location, format!("positive integer"), input.type_name().to_string()))
    }
}

pub fn uint_range(input: mlua::Value, range: std::ops::Range<usize>, location: String) -> Result<Option<usize>, AppError> {
    let error_string = format!("nil, positive number in [{}, {})", range.start, range.end);
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Integer(int) => {
            if int < 0 {
                return Err(AppError::ConfigType(location, error_string, input.type_name().to_string()))
            }

            let unsigned = int as usize;
            if range.contains(&unsigned) {
                return Ok(Some(unsigned))
            }

            Err(AppError::ConfigType(location, error_string, input.type_name().to_string()))
        }
        _ => Err(AppError::ConfigType(location, error_string, input.type_name().to_string()))
    }
}

pub fn int(input: mlua::Value, location: String) -> Result<Option<i64>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Integer(int) => Ok(Some(int)),
        _ => Err(AppError::ConfigType(location, format!("nil, number"), input.type_name().to_string()))
    }
}

pub fn int_necessary(input: mlua::Value, location: String) -> Result<i64, AppError> {
    match input {
        mlua::Value::Integer(int) => Ok(int),
        _ => Err(AppError::ConfigType(location, format!("number"), input.type_name().to_string()))
    }
}

pub fn bool(input: mlua::Value, location: String) -> Result<Option<bool>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Boolean(b) => Ok(Some(b)),
        _ => Err(AppError::ConfigType(location, format!("nil, boolean"), input.type_name().to_string()))
    }
}

pub fn bool_necessary(input: mlua::Value, location: String) -> Result<bool, AppError> {
    match input {
        mlua::Value::Boolean(b) => Ok(b),
        _ => Err(AppError::ConfigType(location, format!("boolean"), input.type_name().to_string()))
    }
}

pub fn func(input: mlua::Value, location: String) -> Result<Option<mlua::Function>, AppError> {
    match input {
        mlua::Value::Nil => Ok(None),
        mlua::Value::Function(func) => Ok(Some(func)),
        _ => Err(AppError::ConfigType(location, format!("nil, function"), input.type_name().to_string()))
    }
}

pub fn table_necessary(input: mlua::Value, location: String) -> Result<mlua::Table, AppError> {
    match input {
        mlua::Value::Table(contents) => Ok(contents),
        _ => Err(AppError::ConfigType(location, format!("table"), input.type_name().to_string()))
    }
}
