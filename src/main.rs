/*
    Sources
    https://doc.rust-lang.org/book/
    https://en.wikipedia.org/wiki/Letter_frequency 
    https://en.wikipedia.org/wiki/Base64
    https://github.com/vijithassar/cryptopals-literate-python/blob/master/challenge06.py.md -- miss understanding about normalized, see what the key should be.
    openssl library provided with Cargo installer. 
    https://docs.rs/openssl/latest/openssl/
    https://docs.openssl.org/master/man3/
    http library provided with Cargo installer.
    https://docs.rs/http/latest/http/
    json library provided with Cargo installer
    https://docs.rs/json/latest/json/
    https://en.wikipedia.org/wiki/Argon2
*/

mod word_frequency;
mod filehelper;
mod encoding;
mod utils;
mod httphandler;
mod labserver;
mod labclient;

pub use crate::filehelper::file_helper;
pub use crate::word_frequency::wfreq;
pub use crate::encoding::enc;
pub use crate::utils::qol;
pub use crate::httphandler::http_handler;
pub use core::time;


fn has_arg(arg : &str, args : &Vec<String>) -> bool{
    let lookfor = "--".to_string()+arg;
    for p in args{
        if *p == lookfor
        {
            return true;
        }        
    }

    return false;
}

fn get_arg(arg : &str, args: &Vec<String>) -> String {
    let lookfor = "--".to_string()+arg;

    let mut found = false;
    for p in args{
        if(found){
            return p.clone();
        }
        if(*p == lookfor){
            found = true;
        }
    }

    panic!("Could not find the key {{{}}}",arg);
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    if(has_arg("server", &args)){
        labserver::lab_server::run(get_arg("address", &args),get_arg("port", &args));
    }else{
        labclient::lab_client::run(get_arg("address", &args),get_arg("port", &args));
    }
}