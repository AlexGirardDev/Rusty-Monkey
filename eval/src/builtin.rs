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
        build_builtin("rest", builtin_rest),
        build_builtin("push", builtin_push),
    ];
    builtins.into_iter().collect()
}

fn build_builtin(key: impl Into<String>, fnn: BuiltinFn) -> (String, Rc<Object>) {
    (key.into(), Object::Builtin(fnn).into())
}

fn builtin_rest(vals: &[Rc<Object>]) -> EvalResponse {
    validate_param_count(1, vals.len())?;
    let slice = get_array(vals[0].clone())?
        .get(1..) 
        .ok_or_else(|| EvalError::IndexOutOfBounds { index: 0, max: 0 })?
        .to_vec();
    Ok(Object::Array(slice).into())
}

fn builtin_push(vals: &[Rc<Object>]) -> EvalResponse {
    validate_param_count(2, vals.len())?;
    let mut slice = get_array(vals[0].clone())?;
    slice.push(vals[1].clone());
    Ok(Object::Array(slice).into())
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
    validate_param_count(1, vals.len())?;
    Ok(get_array(vals[0].clone())?
        .first()
        .ok_or_else(|| EvalError::IndexOutOfBounds { index: 0, max: 0 })?
        .clone())
}

fn builtin_last(vals: &[Rc<Object>]) -> EvalResponse {
    validate_param_count(1, vals.len())?;
    Ok(get_array(vals[0].clone())?
        .last()
        .ok_or_else(|| EvalError::IndexOutOfBounds { index: 0, max: 0 })?
        .clone())
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
fn get_array(obj: Rc<Object>) -> Result<Vec<Rc<Object>>, EvalError> {
    if let Object::Array(a) = obj.as_ref() {
        Ok(a.clone())
    } else {
        Err(EvalError::InvalidOperation {
            operation: "first".to_string(),
            object_type: obj.to_string(),
        })
    }
}
