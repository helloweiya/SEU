use js_sandbox::{Script};
use serde::{Deserialize, Serialize};
const ENCRYPT_FUN:&str = "encryptAES";
#[derive(Serialize,Deserialize)]
struct Args {
    pub data: String,
    pub _p1: String,
}

pub fn encrypt_aes(data: &str, salt: &str) -> String{
    let args = Args{
        data: data.to_string(),
        _p1: salt.to_string()
    };
    let mut js = Script::from_file("ids-encrypt.js").expect("loading js failed!");
    let res:String = js.call(ENCRYPT_FUN, &args).expect("js call failed!");
    res
}