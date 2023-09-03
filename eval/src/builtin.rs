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
    if let Object::String(s) = vals[0].as_ref() {
        Ok(Object::Int(s.len() as i64).into())
    } else {
        Err(generate_param_error(&["String"], vals))
    }
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
