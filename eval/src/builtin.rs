use std::{collections::HashMap, rc::Rc};

// pub type BuiltinFn = fn(&[Rc<Object>]) -> EvalResponse;
use crate::{
    eval::EvalResponse,
    eval_error::EvalError,
    object::{BuiltinFn, Object},
};

pub fn get_builtin_fns() -> HashMap<String, Rc<Object>> {
    let builtins = vec![
        build_builtin("len", builtin_len),
        build_builtin("first", builtin_first),
        build_builtin("last", builtin_last),
    ];
    builtins.into_iter().collect()
}

fn build_builtin(key: impl Into<String>, fnn: BuiltinFn) -> (String, Rc<Object>) {
    (key.into(), Object::Builtin(fnn).into())
}

fn builtin_len(vals: &[Rc<Object>]) -> EvalResponse {
    validate_param_count(1, vals.len())?;
    let len = match vals[0].as_ref() {
        Object::String(s) => s.len(),
        Object::Array(a) => a.len(),
        v => {
            return Err(EvalError::InvalidOperation {
                operation: "len".to_string(),
                object_type: v.to_string(),
            })
        }
    };
    Ok(Object::Int(len as i64).into())
}

fn builtin_first(vals: &[Rc<Object>]) -> EvalResponse {
    if let Object::Array(a) = vals[0].as_ref() {
        let result = a
            .first()
            .ok_or_else(|| EvalError::IndexOutOfBounds { index: 0, max: 0 })?;
        Ok(result.clone())
    } else {
        Err(EvalError::InvalidOperation {
            operation: "first".to_string(),
            object_type: vals[0].to_string(),
        })
    }
}

fn builtin_last(vals: &[Rc<Object>]) -> EvalResponse {
    validate_param_count(1, vals.len())?;
    if let Object::Array(a) = vals[0].as_ref() {
        let result = a
            .last()
            .ok_or_else(|| EvalError::IndexOutOfBounds { index: 0, max: 0 })?;
        Ok(result.clone())
    } else {
        Err(EvalError::InvalidOperation {
            operation: "last".to_string(),
            object_type: vals[0].to_string(),
        })
    }
}

fn generate_param_error(exp: &[&str], actual: &[Rc<Object>]) -> EvalError {
    EvalError::InvalidParamTypes {
        expected: exp.join(","),
        actual: actual
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
    }
}
fn validate_param_count(expected: usize, actual: usize) -> Result<(), EvalError> {
    if expected != actual {
        return Err(EvalError::InvalidParamCount { expected, actual });
    }
    Ok(())
}
