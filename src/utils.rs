#[path = "./encoding.rs"]
pub mod encoding;


pub mod qol{
    
    pub use super::encoding::enc;

     //Seperates two vectors until it finds the value seperator, seperator removed and left side becomes the return vector!
     pub fn seperate_bytes(vec : &mut Vec<u8>, seperator : u8) -> Vec<u8>{
        let mut left : Vec<u8> = Vec::new();
        while !vec.is_empty() {
            let v = vec.remove(0);
            if v == seperator {
                break;
            }
            left.push(v);
        }
        return left;
    }

    //Same length
    pub fn hamming_distance(_str1 : &String, _str2 : &String) -> usize{        
        let _first = enc::utf8charstobytes(&_str1);
        let _second = enc::utf8charstobytes(&_str2);
        
        return hamming_distance_bytes(&_first, &_second);
    }

    pub fn hamming_distance_bytes(_str1 : &Vec<u8>, _str2 : &Vec<u8>) -> usize{
        let mut res : usize = 0;
        
        let mut _first = _str1.clone();
        let mut _second = _str2.clone();
        let _filter : u8 = 1;
        while _first.len() > 0 && _second.len() > 0 {
            let mut _firstbyte = _first.pop().unwrap();
            let mut _secondbyte = _second.pop().unwrap();

            for _i in 0..8{
                if (_firstbyte & _filter) != (_secondbyte & _filter) {
                    res = res + 1;
                }
                
                _firstbyte = _firstbyte >> 1;
                _secondbyte = _secondbyte >> 1;
            }
        }

        return res;
    }

    //True if could increment without increasing size, otherwise False.
    pub fn incrementbytearray(mut _arr : Vec<u8>) -> Vec<u8>{
        if _arr.len() == 0 {
            _arr.push(0);
        }else{
            let mut index = _arr.len()-1;
            loop{
                if _arr[index] == (0xff as u8) {
                    _arr[index] = 0;
                }else{
                    _arr[index] = _arr[index] + 1;
                    break;
                }

                if index == 0 {
                    index = _arr.len();
                    break;
                }else{
                    index = index - 1;
                }
            }
            if index == _arr.len() {
                _arr.insert(0,1);
            }
        }
        return _arr;
    }

    pub fn sample_variance(values : &Vec<f64>) -> f64{

        let mut mean = 0.0;
        for v in values.clone(){
            mean = mean + v;
        }
        mean = mean / (values.len() as f64);

        let mut var = 0.0;

        for v in values.clone(){
            var = var + (v-mean)*(v-mean);
        }



        return var / (values.len() as f64);
    }
}



//this is a test
//wokka wokka!!!