use crate::env::Env;
use crate::ffi::*;
use deno_core::v8;

#[no_mangle]
pub unsafe extern "C" fn napi_is_detached_arraybuffer(
  env: napi_env,
  value: napi_value,
  result: *mut bool,
) -> napi_status {
  let mut env = &mut (env as *mut Env);
  let value: v8::Local<v8::Value> = std::mem::transmute(value);
  let ab = v8::Local::<v8::ArrayBuffer>::try_from(value).unwrap();
  // TODO: what is API for checking if ArrayBuffer is detached?
  // there's only is_detachable I could find.
  *result = false;
  napi_ok
}
