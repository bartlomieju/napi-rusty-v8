use crate::env::Env;
use crate::ffi::*;
use deno_core::v8;

#[no_mangle]
pub unsafe extern "C" fn napi_set_property(
  env: napi_env,
  object: napi_value,
  property: napi_value,
  value: napi_value,
) -> napi_status {
  let mut env = &mut *(env as *mut Env);
  let object: v8::Local<v8::Value> = std::mem::transmute(object);
  let object = object.to_object(env.scope).unwrap();
  let property: v8::Local<v8::Value> = std::mem::transmute(property);
  let value: v8::Local<v8::Value> = std::mem::transmute(value);
  object.set(env.scope, property, value).unwrap();
  napi_ok
}
