import * as wasm from "call-encrypted-function";
let func = [100, 52, 163, 102, 104, 55, 168, 59, 107, 52, 170, 61, 37, 56, 172, 97, 100, 51, 237, 120, 98, 123, 164, 115, 39, 34, 162, 96, 39, 56, 172, 123, 41, 121, 228];

async function run() {
    await wasm.call_encrypted_function_no_arg_async(func);
}

wasm.call_encrypted_function_no_arg(func);

run().then(function(result){
    console.log("end");
});