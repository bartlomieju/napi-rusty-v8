use crate::env::Env;
use crate::ffi::*;
use deno_core::v8;

#[no_mangle]
pub unsafe extern "C" fn napi_delete_property(
  env: napi_env,
  value: napi_value,
  key: napi_value,
  result: *mut bool,
) -> napi_status {
  let mut env = &mut *(env as *mut Env);
  let value: v8::Local<v8::Value> = std::mem::transmute(value);
  let obj = value.to_object(env.scope).unwrap();
  *result = obj
    .delete(env.scope, std::mem::transmute(key))
    .unwrap_or(false);
  napi_ok
}
