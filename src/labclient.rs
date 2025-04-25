//https://en.wikipedia.org/wiki/Argon2 for parameters

#[path = "./Tcp_Stream_Handler.rs"]
mod  Tcp_Stream_Handler;
pub mod lab_client{

    use std::{io::{Read, Write}, net::TcpStream};
    
    use json::JsonValue::Null;
    use openssl::{bn::{BigNum, BigNumContext}, envelope::Open, string::OpensslString, symm::Cipher, symm::encrypt, symm::decrypt};

    use crate::{enc, labclient::Tcp_Stream_Handler::TcpStreamHandler::{self, StreamMessage}};

    pub fn run(_address : String,port : String){
        let mut context: &mut BigNumContext = &mut BigNumContext::new_secure().unwrap();        

        println!("Starting client!");
        let mut address : String = _address;
        address = address + ":" + port.as_str();

        println!("Trying to connect to the adress {{{}}}",address);
        
        let listenerres = std::net::TcpStream::connect(address.clone());
        if(listenerres.is_err()){
            panic!("Received error when trying to connect {{{}}}",listenerres.unwrap_err());
        }
        println!("Connected!");
        let mut stream = listenerres.unwrap();
        let mut message : StreamMessage;
        
        println!("Reading parameter g!");
        message = TcpStreamHandler::readMessage(&mut stream);
        let mut generator = BigNum::from_slice(message.data.as_slice()).unwrap();
        println!("g is {{{}}}",generator);

        println!("Reading parameter p!");
        message = TcpStreamHandler::readMessage(&mut stream);
        let mut prime = BigNum::from_slice(message.data.as_slice()).unwrap();
        println!("p is {{{}}}",prime);

        println!("Reading parameter g^a!");
        message = TcpStreamHandler::readMessage(&mut stream);
        let mut pubkey = BigNum::from_slice(message.data.as_slice()).unwrap();
        println!("g^a is {{{}}}",pubkey);

        let keyinstance = openssl::dh::Dh::from_pqg(prime,None,generator).unwrap();
        let key = keyinstance.generate_key().unwrap();
        
        println!("Sending our public key, {{{}}}!",key.public_key());
        TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&key.public_key().to_vec()));

        
        let mut salt : [u8;32] = [0;32];
        let mut sharedsecret = key.compute_key(&pubkey).unwrap();
        let mut sharedkey : [u8;32] = [0;32];
        let mut sharediv : [u8;16] = [0;16];
        match openssl::kdf::argon2id(None, sharedsecret.as_mut_slice(), salt.as_mut_slice(), None, None, 1, 1, 32, &mut sharedkey) {
            Ok(_v) => {}
            Err(v) => {panic!("Could not create a sharedkey, argon2id err = {{{}}}",v)}
        }

        match openssl::kdf::argon2id(None, sharedsecret.as_mut_slice(), salt.as_mut_slice(), None, None, 1, 1, 32, &mut sharediv) {
            Ok(_v) => {}
            Err(v) => {panic!("Could not create a sharediv, argon2id err = {{{}}}",v)}
        }
        println!("Shared secret is {{{}}} with {{{}}} bits",enc::bytestohexstr(&sharedkey.to_vec()),sharedkey.len()*8);
        println!("Shared iv is {{{}}} with {{{}}} bits",enc::bytestohexstr(&sharediv.to_vec()),sharediv.len()*8);

    
        while true {
            let mut buffer = String::new();
            let stdin = std::io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer);
            while buffer.ends_with("\r") || buffer.ends_with("\n"){
                buffer.pop();
            }
            println!("Sending : \n      {{{}}}",buffer);
            let mut content = buffer.as_bytes().to_vec();
            let mut newiv = TcpStreamHandler::newIV();

            //Add the newiv to the content!
            for i in newiv{
                content.push(i);
            }

            let mut tag = TcpStreamHandler::encryptContent(&mut content, &sharedkey, &mut sharediv, &salt);

            TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&content));
            TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&tag.to_vec()));

            sharediv = newiv;

            println!("New iv = {{{}}}",enc::bytestohexstr(&sharediv.to_vec()));


            message = TcpStreamHandler::readMessage(&mut stream);
            content = message.data;

            message = TcpStreamHandler::readMessage(&mut stream);
            for i in 0..16{
                tag[i] = message.data[i];
            }

            TcpStreamHandler::decryptContent(&mut content, &sharedkey, &mut sharediv, &salt, &tag);

            for p in 0..16{
                sharediv[15-p] = content.remove(content.len()-1);
            }
            


            println!("Received : \n     {{{}}}",String::from_utf8(content).unwrap());

            println!("New iv = {{{}}}",enc::bytestohexstr(&sharediv.to_vec()));
        }

    }
}