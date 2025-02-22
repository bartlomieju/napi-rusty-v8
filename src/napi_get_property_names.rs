use crate::env::Env;
use crate::ffi::*;
use deno_core::v8;

#[no_mangle]
pub unsafe extern "C" fn napi_get_property_names(
  env: napi_env,
  object: napi_value,
  result: *mut napi_value,
) -> napi_status {
  let mut env = &mut *(env as *mut Env);
  let object: v8::Local<v8::Value> = std::mem::transmute(object);
  let array: v8::Local<v8::Array> = object
    .to_object(env.scope)
    .unwrap()
    .get_property_names(env.scope)
    .unwrap();
  let value: v8::Local<v8::Value> = array.into();
  *result = std::mem::transmute(value);
  napi_ok
}
