//https://en.wikipedia.org/wiki/Argon2 for parameters

#[path = "./Tcp_Stream_Handler.rs"]
mod  Tcp_Stream_Handler;
pub mod lab_server{
    use std::io::{Read, Write};

    use openssl::{bn::{BigNum, BigNumContext}, envelope::Open, string::OpensslString, symm::Cipher, symm::encrypt, symm::decrypt};

    use crate::{enc, labserver::Tcp_Stream_Handler::TcpStreamHandler};

    pub fn run(_address : String,port : String){
        println!("Starting server!");
        println!("Generating keys!");
        let keyinstance = openssl::dh::Dh::get_2048_256().unwrap();
        let keyres = keyinstance.generate_key();
        let key = keyres.unwrap();
        let mut context: &mut BigNumContext = &mut BigNumContext::new_secure().unwrap();

        println!("Generated keys!");

        let mut address : String = _address;
        address = address + ":" + port.as_str();

        println!("Server trying to listen on {{{}}}",address);

        let listenerres = std::net::TcpListener::bind(address.clone());
        if(listenerres.is_err()){
            panic!("Could not listen on {{{}}}, Err = {{{}}}",address,listenerres.unwrap_err());
        }
        

        let listner = listenerres.unwrap();

        println!("Now listening on {{{}}}",address);

        println!("Waiting for client to connect!");
    
        let socketres = listner.accept();
        if(socketres.is_err()){
            panic!("Socket error when trying to establish, err = {{{}}}",socketres.unwrap_err());
        }

        let sockettuple = socketres.unwrap();
        let mut stream = sockettuple.0;
        
        println!("Pushing the parameter g, {{{}}}",key.generator());
        TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&key.generator().to_vec()));
        
        println!("Pushing the parameter p, {{{}}}",key.prime_p());
        TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&key.prime_p().to_vec()));

        println!("Pushing the parameter g^a, {{{}}}",key.public_key());
        TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&key.public_key().to_vec()));

        let mut message : TcpStreamHandler::StreamMessage;
        message = TcpStreamHandler::readMessage(&mut stream);
        let mut theirkey = BigNum::from_slice(&message.data).unwrap();

        println!("clients public key is : {{{}}}",theirkey);      

        let mut salt : [u8;32] = [0;32];
        let mut sharedsecret = key.compute_key(&theirkey).unwrap();
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
        
        while true{
            message = TcpStreamHandler::readMessage(&mut stream);
            let mut content = message.data;
            message = TcpStreamHandler::readMessage(&mut stream);
            let mut tag : [u8;16] = [0;16];
            for i in 0..16{
                tag[i] = message.data[i];
            }

            TcpStreamHandler::decryptContent(&mut content, &sharedkey, &mut sharediv, &salt, &tag);

            //Get newiv
            for p in 0..16{
                sharediv[15-p] = content.remove(content.len()-1);
            }



            println!("Received : \n     {{{}}}",String::from_utf8(content.clone()).unwrap());
            println!("New iv = {{{}}}",enc::bytestohexstr(&sharediv.to_vec()));

            let mut string = String::from_utf8(content).unwrap();
            string = "I have received ".to_owned() + string.as_str();

            content = string.as_bytes().to_vec();

            let mut newiv = TcpStreamHandler::newIV();

            //Add the newiv to the content!
            for i in newiv{
                content.push(i);
            }


            tag = TcpStreamHandler::encryptContent(&mut content, &sharedkey, &mut sharediv, &salt);

            TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&content));
            TcpStreamHandler::sendMessage(&mut stream, TcpStreamHandler::createMessage(&tag.to_vec()));

            sharediv = newiv;
            println!("New iv = {{{}}}",enc::bytestohexstr(&sharediv.to_vec()));


        }
        


    }
}