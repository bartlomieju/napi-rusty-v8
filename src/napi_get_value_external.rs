use crate::env::Env;
use crate::ffi::*;
use deno_core::v8;

#[no_mangle]
pub unsafe extern "C" fn napi_get_value_external(
  env: napi_env,
  value: napi_value,
  result: *mut *mut c_void,
) -> napi_status {
  let mut env = &mut *(env as *mut Env);
  let value: v8::Local<v8::Value> = std::mem::transmute(value);
  let ext = v8::Local::<v8::External>::try_from(value).unwrap();
  *result = ext.value();
  napi_ok
}
