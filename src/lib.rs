use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const KEY: u32 = 123456789;

fn decrypt_js_from_string(func: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let mut index = 0;
    let key_bytes = KEY.to_be_bytes();
    for char in func {
        result.push(char ^ key_bytes[index]);
        index = (index + 1).rem_euclid(4);
    }
    Ok(String::from_utf8(result)?)
}

#[wasm_bindgen]
pub fn call_encrypted_function_no_arg_async(func: &[u8]) -> js_sys::Promise {
    let js_function_encrypted = func.to_owned();

    let task = async move {
        match decrypt_js_from_string(&js_function_encrypted) {
            Ok(func) => {
                Ok(js_sys::Function::new_no_args(&func).call0(&JsValue::undefined())?)
            },
            Err(e) => Err(JsValue::from(e.to_string()))
        }
    };

    wasm_bindgen_futures::future_to_promise(task)
}

#[wasm_bindgen]
pub fn call_encrypted_function_no_arg(enc_func: &[u8]) -> Result<JsValue, JsValue> {
    let func = match decrypt_js_from_string(enc_func) {
        Ok(func) => {
            js_sys::Function::new_no_args(&func)
        },
        Err(e) => return Err(JsValue::from(e.to_string()))
    };
    Ok(func.call0(&JsValue::undefined())?)
}
