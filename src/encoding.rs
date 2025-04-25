

pub mod enc{
    pub fn hextou8(_c : char) -> u8{
        let mut v : u8 = _c as u8;
        v = v - ('0' as u8);
        if v > (9 as u8) && (_c as u8) < ('a' as u8) {
            let diff = ('A' as u8) - ('0' as u8);
            v = v - diff;
            v = v + 10;
        }
    
        if v > (9 as u8) && (_c as u8) >= ('a' as u8) {
            let diff = ('a' as u8) - ('0' as u8);
            v = v - diff;
            v = v + 10;
        }
        
        
        
        return v;
    }

    pub fn base64tou8(_c : char) -> u8{
        for i in 0..64 {
            if u8tobase64(i) == _c {
                return i as u8;
            }
        }
        return 64;
    }

    pub fn hexstrtobytes(_str : String) -> Vec<u8>{
        let mut strcopy = _str.clone();
        let mut vec : Vec<u8> = Vec::with_capacity(strcopy.len() / 2); // /2 since two hex decimals equal to one utf8 character. 
        
        while strcopy.len() > 0 {
            let mut byte = hextou8(strcopy.remove(0)) << 4;
            byte = byte + hextou8(strcopy.remove(0));
            
            vec.push(byte);
        }
        
        return vec;
    }

    pub fn u8tobase64(_val : u8) -> char{

        let bytes = _val.to_be_bytes();
        let byte = bytes[0] as u8;
        if byte <= 25 {
            return (('A' as u8) + byte) as char;
        }
    
        if byte <= 51 {
            return (('a' as u8) + byte - 26) as char;
        }
    
        if byte <= 61 {
            return (('0' as u8) + byte - 52) as char;
    
        }
        if byte == 62 {
            return '+';
        }
        if byte == 63 {
            return '/';
        }
    
        return 'A';
    }

    pub fn u8tohex(mut _u8 : u8) -> char{
        if _u8 < 10 {
            return ((_u8 ) + ('0' as u8)) as char;
        }
        return ((_u8 - 10) + ('A' as u8)) as char;
    }

    pub fn u8tobinary(mut _u8 : u8) -> String{
        let mut _str = String::new();
        let mut u8base = 1;
        for i in 1..9{
            let mut _val = String::from("0");
            if _u8 & u8base != 0 {
                _val = String::from("1")
            }
    
            _str = _val + &_str;
    
            if i != 8 {
                u8base = u8base*2;
            }
        }
        return _str;
    }

    pub fn bytestohexstr(vec : &Vec<u8>) -> String{
        let mut _str = String::with_capacity((vec.len()*2) as usize);
        
        for i in 0..vec.len(){
            let _bits = vec[i];
            let lefthex = (_bits & 0xf0) >> 4;
            let righthex = _bits & 0x0f;
        
    
            _str.push(u8tohex(lefthex) as char);
            _str.push(u8tohex(righthex) as char);
        }
    
        return _str;
    }

    pub fn base64tobytes(_str : &String) -> Vec<u8>{
        let mut res : Vec<u8> = Vec::new();
        let mut arr : u16 = 0;
        let mut bits : u8 = 0;
        let mut chars : usize = 0;
        let mut _filter : u16 = 0xff00;
        while chars < _str.len() || bits >= 8 {
            while bits < 8 {
                let part = base64tou8(_str.chars().nth(chars).unwrap()) as u16;
                arr = arr + (part << (16-6-bits));
                chars += 1;
                bits += 6;
            }
            let byte = _filter & arr;
            bits -= 8;
            arr = arr << 8;
            res.push((byte >> 8) as u8);
        }
        return res;
    }

    pub fn hexto64(_string: &String) -> String{
        //one hex = 4 bits, half a byte, 
        //base64 six bits, 
        //Padding is in the front, size we push at the font
        let mut arr : u16 = 0;
        let mut res : String = String::new();
        let mut bits : u8 = 0;
        let mut chars = 0;
        let mut _filter : u16 = 64512; //2^15 + 2^14... but not 2^9 + 2^8.... 
        while chars < _string.len() || bits >= 6 {
           while bits < 6 {
                let part = hextou8(_string.chars().nth(chars).unwrap()) as u16;
                arr = arr + (part << (16-4-bits));
                chars += 1;
                bits += 4;
           }
    
           let mut _val = arr & _filter;
           arr = arr << 6;
           bits = bits - 6;
    
           _val = _val >> (10);
    
           res.push(u8tobase64(_val as u8));
        }
    
       
        return res;
    }

    pub fn utf8charstobytes(_str : &String) -> Vec<u8>{
        let mut res : Vec<u8> = Vec::new();
        for _char in _str.chars(){
            res.push(_char as u8);
        }
        return res;
    }
}


//49276D206B696C6C696E6720796F757220627261696E206C696B65206120706F69736F6E6F7573206D757368726F6F6D
//49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d