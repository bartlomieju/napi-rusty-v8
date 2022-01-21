use crate::env::Env;
use crate::ffi::*;
use deno_core::v8;

#[no_mangle]
pub unsafe extern "C" fn napi_create_array_with_length(
  env: napi_env,
  len: i32,
  result: *mut napi_value,
) -> napi_status {
  let mut env = &mut *(env as *mut Env);
  let value: v8::Local<v8::Value> = v8::Array::new(env.scope, len).into();
  *result = std::mem::transmute(value);
  napi_ok
}
