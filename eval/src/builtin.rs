use std::{collections::HashMap, rc::Rc};

// pub type BuiltinFn = fn(&[Rc<Object>]) -> EvalResponse;
use crate::{
    eval::EvalResponse,
    eval_error::EvalError,
    object::{BuiltinFn, Object},
};

pub fn get_builtin_fns() -> HashMap<String, Rc<Object>> {
    let builtins = vec![build_builtin("len", builtin_len)];
    builtins.into_iter().collect()
}

fn build_builtin(key: impl Into<String>, fnn: BuiltinFn) -> (String, Rc<Object>) {
    (key.into(), Object::Builtin(fnn).into())
}

fn builtin_len(vals: &[Rc<Object>]) -> EvalResponse {
    if vals.len() != 1 {
        return Err(generate_param_error(&["String"], vals));
    }
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
fn generate_param_error(exp: &[&str], actual: &[Rc<Object>]) -> EvalError {
    EvalError::InvalidParams {
        expected: exp.join(","),
        actual: actual
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
    }
}
