
pub mod http_handler{
    use core::str;
    use std::{io::{Read, Write}, net::TcpStream, str::FromStr, time::Duration};

    

    use crate::qol;

    pub fn version_tostring(version : http::Version) -> String{
       
        if version == http::Version::HTTP_09 {
            return "HTTP/0.9".to_string();
        }

        if version == http::Version::HTTP_10 {
            return "HTTP/1".to_string();
        }
        
        if version == http::Version::HTTP_2 {
            return "HTTP/2".to_string();
        }

        if version == http::Version::HTTP_3 {
            return "HTTP/3".to_string();
        }

        

        return "HTTP/1.1".to_string();
    } 

    pub fn string_toversion(version : String) -> http::Version
    {
        let values = [http::Version::HTTP_09,http::Version::HTTP_10,http::Version::HTTP_11,http::Version::HTTP_2,http::Version::HTTP_3];
        for v in values {
            if version == version_tostring(v){
                return v;
            }
        }

        panic!("Could not match to any values, string_toversion {}!",version);
    }

    fn packet_append(packet : &mut Vec<u8>, string : String){
        for b in string.bytes() {
            packet.push(b);
        }
    }

    type ReadStreamResult = Result<Vec<u8>, String>;
    /*
    until is priority based, lowest index -> highest priority to match for
    */
    fn stream_readline(stream : &mut TcpStream) -> ReadStreamResult{
        let mut read : Vec<u8> = Vec::new();
        loop{
            let mut carr = [0 as u8];
            let readres= stream.read_exact(&mut carr);
                
                if readres.is_err() {
                    panic!("{{{}}}",readres.unwrap_err());
                }
            let c = carr[0];
                if c == '\n' as u8 {
                    if read.ends_with(&['\r' as u8]) {
                        read.pop();
                    }
                    break;
                }else{
                    read.push(c);
                }
            }
        
        

        Ok(read)
    }   

    pub fn get_response(request: http::Request<()>) -> http::Response<Vec<u8>> {
        let mut packet : Vec<u8> = Vec::new();
        let version = version_tostring(request.version());
        
        //METHOD
        packet_append(&mut packet, request.method().as_str().to_string());
        packet_append(&mut packet, " ".to_string());
        
        //PATH
        packet_append(&mut packet, request.uri().path().to_string());
        packet_append(&mut packet, " ".to_string());

        //VERSION
        packet_append(&mut packet, version);
        packet_append(&mut packet, "\r\n".to_string());
        for key in request.headers().keys(){
            //KEYNAME:
            packet_append(&mut packet, key.as_str().to_string());
            packet_append(&mut packet, ": ".to_string());

            let val  = request.headers().get(key).unwrap();
            for b in val.as_bytes(){
                packet.push(b.clone());
            }
            packet_append(&mut packet, "\r\n".to_string());
        }
        packet_append(&mut packet, "\r\n".to_string());
        
        //let str_eqv = String::from_utf8(packet.clone()).unwrap();
        //println!("HTTP-REQUEST IS : \n{{{}}}",str_eqv);
        
        let mut vec : Vec<u8> = Vec::new();
        let mut resp = http::Response::builder();
        let defaulterrorcode = http::StatusCode::SERVICE_UNAVAILABLE;
        resp = resp.status(defaulterrorcode);
        


        let mut address = request.uri().host().unwrap().to_string();
        address = address + ":"+ request.uri().port().unwrap().as_str();

        let streamres = TcpStream::connect(address.clone());
        
        if streamres.is_err() {
            println!("Connection could not be established to {{{}}}! ERR = {{{}}}",address,streamres.unwrap_err());
            return resp.body(vec).unwrap();
        }else{
            //println!("Connection established to {{{}}}",address);
        }
        

        let mut stream = streamres.unwrap();
        let timeout_time_s = 30;
        match stream.set_read_timeout(Option::from(Duration::new(timeout_time_s, 0))) {
            Ok(_v) => {
                //println!("Set timeout to {{{} seconds}}",timeout_time_s)
            }
            Err(v) => {
                println!("Could not set timeout to {{{} seconds}}, ERR = {{{}}}",timeout_time_s,v)
            }
        }
        
        let res = stream.write_all(packet.as_slice());
        if res.is_err() {

            println!("Was cut off from sending whole packet!");
        }else{
            //println!("Every byte of the packet was pushed as it should, now reading start line!");
            let startlineres = stream_readline(&mut stream);
            if startlineres.is_ok() {
                let mut startline = startlineres.unwrap();

                //println!("Read startline : {{{}}}",String::from_utf8(startline.clone()).unwrap());


                let versionfield =  String::from_utf8(qol::seperate_bytes(&mut startline, ' ' as u8)).unwrap();
                let codefield = String::from_utf8(qol::seperate_bytes(&mut startline, ' ' as u8)).unwrap();
                resp = resp.version(string_toversion(versionfield));
                let code = http::StatusCode::from_str(codefield.as_str()).unwrap();
                
                resp = resp.status(code);


                let mut content_length : usize = 0;
                //Reading headers!
                loop{
                    
                    let lineres = stream_readline(&mut stream);

                    if lineres.is_err() {
                        println!("Could not read header line, ERR = {{{}}}",lineres.unwrap_err());
                        break;
                    }else{
                        let mut line = lineres.unwrap();
                        if line.is_empty() {
                            
                            //println!("Read all header lines, content coming up with a length of : {{{}}}",content_length);
                            break;
                        }
                        //print!("Read header line {{{}}} -> {{",String::from_utf8(line.clone()).unwrap());
                        let left = qol::seperate_bytes(&mut line, ':' as u8);
                        let headerkey = str::from_utf8(left.as_slice()).unwrap();
                        //print!(" {{{}}} = ",headerkey);
                        
                        while line.starts_with(&[' ' as u8]) {
                            line.remove(0);
                        }

                        let headervalue = str::from_utf8(&line.as_slice()).unwrap();
                                                
                        //print!("{{{}}} }}\n",headervalue);
                        resp = resp.header(headerkey, headervalue);
                        if headerkey.to_ascii_lowercase() == "content-length".to_string() {
                            content_length = usize::from_str(headervalue).unwrap();
                        }
                    }
                }

                //Reading content!
                vec.resize(content_length, 0);
                let readbody = stream.read_exact(vec.as_mut_slice());
                if readbody.is_err() {
                    resp = resp.status(defaulterrorcode);
                }
                else{
                    //println!("Read the whole body to {{{}}}",String::from_utf8_lossy(vec.as_slice()))
                }

            }else{
                println!("Could not read startline, ERR = {{{}}}",startlineres.unwrap_err());
            }
        }

        


        match stream.shutdown(std::net::Shutdown::Both)
        {
            Ok(_res) => {
                //println!("Connection closed as it should!");
            }
            Err(_res) => {
                println!("Shutdown procedure not as expected, ERR = {{{}}}",_res);
            }
        }

        return resp.body(vec).unwrap();
    }

    
}