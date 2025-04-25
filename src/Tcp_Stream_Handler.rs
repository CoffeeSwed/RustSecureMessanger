pub mod TcpStreamHandler{
    use core::panic;
    use std::{io::{Read, Write}, net::TcpStream};
    use openssl::{bn::{BigNum, BigNumContext}, envelope::Open, string::OpensslString};
    use rand::{RngCore, SeedableRng};

    use crate::{enc};

        pub fn newIV() -> [u8;16] {
            let mut returning : [u8;16] = [0;16];
            let mut instance = rand::rngs::StdRng::from_entropy();
            instance.fill_bytes(&mut returning);
            return returning;
        }
    
        pub struct StreamMessage{
            pub len : usize,
            pub data : Vec<u8>
        }


        pub fn createMessage(content : &Vec<u8>) -> StreamMessage {



            let mut answer = StreamMessage {
                len : content.len(),
                data : content.clone()
            };

            return answer;
        }

        pub fn sendMessage(stream : &mut TcpStream, message : StreamMessage){
            match stream.write_all(&message.len.to_be_bytes()) {
                Ok(_v) => {
                    println!("Pushed message length {{{}}} to stream!",message.len);
                    match stream.write_all(message.data.as_slice()){
                        Ok(_v) => {
                            println!("Pushed message content {{{}}} to stream!",enc::bytestohexstr(&message.data));
                        }
                        Err(v) => {
                            panic!("Could not push message content, err = {{{}}}",v);
                        }
                    }
                }
                Err(v) => {
                    panic!("Could not push message length, err = {{{}}}",v);
                }
            };
            
        }

        pub fn encryptContent(content : &mut Vec<u8>,key : &[u8;32], iv : &mut [u8;16], salt : &[u8;32]) -> [u8;16]{
            
            let mut res : Vec<u8> = Vec::new();
            let cipher = openssl::cipher::Cipher::aes_256_gcm();
            let mut ctx = openssl::cipher_ctx::CipherCtx::new().unwrap();
            ctx.set_tag_length(16);
            match ctx.encrypt_init(Some(cipher), Some(key), Some(iv)){
                Ok(_v) => {
                    match ctx.cipher_update_vec(content.as_mut_slice(), &mut res){
                        Ok(_v) => {
                            match ctx.cipher_final_vec(&mut res){
                                Ok(__v) => {

                                }
                                Err(_v) => {
                                    panic!("cipher_final_vec Err = {{{}}}",_v);
                                }
                            }
                        }
                        Err(v) => {
                            panic!("Could not cipher_update_vec, Err = {{{}}}",v);
                        }
                    }
                }
                Err(v) => {
                    panic!("Could not encrypt_init, Err = {{{}}}",v);
                }
            }
            
            content.clear();
            for b in res{
                content.push(b);
            }
            let mut tag : [u8;16] = [0;16];
            ctx.tag(&mut tag);
            return tag;
        }

        pub fn decryptContent(content : &mut Vec<u8>,key : &[u8;32], iv : &mut [u8;16], salt : &[u8;32], tag : &[u8;16]){
            let mut res : Vec<u8> = Vec::new();

            let cipher = openssl::cipher::Cipher::aes_256_gcm();
            let mut ctx = openssl::cipher_ctx::CipherCtx::new().unwrap();

            match ctx.decrypt_init(Some(cipher), Some(key), Some(iv.as_slice())){
                Ok(_v) => {
                    ctx.set_tag_length(16);
                    ctx.set_tag(tag);
                    match ctx.cipher_update_vec(content.as_slice(), &mut res) {
                        Ok(_v) => {
                            match ctx.cipher_final_vec(&mut res) {
                                Ok(_v) => {
                                }
                                Err(v) => {
                                    for err in v.errors(){
                                        println!("Error when trying to cipher_final_vec, err = {{{}}}",err);
                                    }
                                    panic!("Exiting cause of {{{}}}",v);
                                }
                            }
                        }
                        Err(_v) => {
                            panic!("Error when trying to update vec, err = {{{}}}",_v);
                        }
                    }
                }
                Err(v) => {
                    panic!("Got error when trying to init Cipherctx! Err= {{{}}}",v);
                }
            }
            content.clear();
            for b in res{
                content.push(b);
            }
        }

        

        pub fn readMessage(stream : &mut TcpStream) -> StreamMessage{
            let mut header : [u8 ; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];

            match stream.read_exact(header.as_mut_slice()){
                Ok(_v) => {
                    let mut length = usize::from_be_bytes(header);
                    println!("Read the message length {{{}}} from stream!",length);
                    let mut vec : Vec<u8> = Vec::with_capacity(length);
                    
                    for i in 0..length{
                        vec.push(0);
                    }

                    match stream.read_exact(vec.as_mut_slice()){
                        Ok(_v) => {
                            println!("Read message content {{{}}} from stream!",enc::bytestohexstr(&vec));

                            let mut iv : [u8;16] = [0;16];

                            return StreamMessage {
                                len : length,
                                data : vec
                            };

                        }
                        Err(v) => {
                            panic!("Could not read message content, err = {{{}}}",v);
                        } 
                    }

                }
                Err(v) => {
                    panic!("Could not read message length, err = {{{}}}",v);
                }
            }

            panic!("Could not read message from socket!");
        }
}