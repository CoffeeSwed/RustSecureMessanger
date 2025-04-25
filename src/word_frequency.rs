/*
    https://doc.rust-lang.org/book/
    https://en.wikipedia.org/wiki/Letter_frequency 
    https://en.wikipedia.org/wiki/Base64
    https://www.wordfrequency.info/samples.asp Word database
    https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html Reading lines
    https://doc.rust-lang.org/std/collections/struct.HashMap.html
    https://github.com/vijithassar/cryptopals-literate-python/blob/master/challenge06.py.md
*/
#[path = "./filehelper.rs"]
pub mod filehelper;


pub mod wfreq{
    pub use super::filehelper::file_helper;

    
    
    
    use std::collections::HashMap;
    
    pub struct WfreqInstance {
        words_hash : HashMap<String,Vec<u8>>,
        letters : Vec<f64>
    }

    pub fn create_instance() -> WfreqInstance{
        let instance = WfreqInstance{
            letters : getdistrubtiontable(),
            words_hash : loadworddatabase("words.csv")
        };
        return instance;
    }

    pub fn containssequence(_vec : &Vec<u8>, _seq : &Vec<u8>) -> usize{


        return _vec.len();
    }


    pub fn loadworddatabase(loc : &str) -> HashMap<String,Vec<u8>>{
        let mut words : HashMap<String,Vec<u8>> = HashMap::new();

        let results = file_helper::create_line_iterator(loc);
        match results{
            Ok(results) => {
                for line in results.flatten(){
                    
                    let mut v : Vec<u8> = Vec::new();
                    for b in line.as_bytes(){
                        v.push(*b);
                    }
                    words.insert(line,v);
                }
            }
            Err(v) => {
                panic!("Could not read file {{{}}}, error = {{{}}}!",loc,v);
            }
        }

        return words;
    }

    pub fn getdistrubtiontable() -> Vec<f64>{
        let mut vec : Vec<f64> = Vec::with_capacity(256); //Ascii table
        for _i in 0..=256 {
            vec.push(-2.0 as f64);
            
        }
        vec['A' as usize]  = 8.2;
        vec['B' as usize] = 1.5;
        vec['C' as usize] = 2.8;
        vec['D' as usize] = 4.3;
        vec['E' as usize] = 12.7;
        vec['F' as usize] = 2.2;
        vec['G' as usize] = 2.0;
        vec['H' as usize] = 6.1;
        vec['I' as usize] = 7.0;
        vec['J' as usize] = 0.15; 
        vec['K' as usize] = 0.77;
        vec['L' as usize] = 4.0;
        vec['M' as usize] = 2.4;
        vec['N' as usize] = 6.7;
        vec['O' as usize] = 7.5;
        vec['P' as usize] = 1.9;
        vec['Q' as usize] = 0.095;
        vec['R' as usize] = 6.0;
        vec['S' as usize] = 6.3;
        vec['T' as usize] = 9.1;
        vec['U' as usize] = 2.8;
        vec['V' as usize] = 0.98;
        vec['W' as usize] = 2.4;
        vec['X' as usize] = 0.15;
        vec['Y' as usize] = 2.0;
        vec['Z' as usize] = 0.074;
    
        vec['*' as usize] = -1.0;
        vec['-' as usize] = -1.0;
        vec['\n' as usize] = 0.0;
        vec[' ' as usize] = 0.0;
        
    
        for i in ('a' as usize)..('z' as usize + 1){
            vec[i] = vec['A' as usize + (i - 'a' as usize)];
        }
        return vec;
    }

    pub fn getscoreofword(_str : &String, _instance : &WfreqInstance) -> f64{
        if _instance.words_hash.contains_key(_str) {
            return _str.len() as f64;
        }
        return 0.0;
    }
    
    pub fn getscoreofasciistring(_str : &Vec<u8>, _instance : &WfreqInstance) -> f64{
        

        let scoretable : &Vec<f64> = &_instance.letters;
        let mut score : f64 = 0.0;
        let mut copy : Vec<u8> = Vec::with_capacity(_str.len());
        for i in 0.._str.len(){
            score = score + scoretable[_str[i] as usize];
            
            copy.push(_str[i]);
        }

        let sentence = String::from_utf8_lossy(copy.as_slice());
        
        if sentence == "" {
            return -2.0;
        }

        let chars = sentence.chars();
        let mut word = String::new();
        for _char in chars{
            let mut _copy = _char;
            _copy.make_ascii_lowercase();
            if _char.is_ascii_alphanumeric() {
                word.push(_copy);
            }else{
                score = score + 2.0*getscoreofword(&word,_instance);
                word = String::new();
            }


        }
        score = score + 2.0*getscoreofword(&word,_instance);



        return score;
    }
}

