/*
https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
*/

pub mod file_helper{
    use std::fs::File;
    use std::io::{self, BufRead};
    

    pub fn create_line_iterator(loc : &str) -> io::Result<io::Lines<io::BufReader<File>>>
    {
        let file = open_file(loc)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn open_file(loc: &str) -> io::Result<File> {
        let file = File::open(loc);
        return file;
    }

    pub fn create_file(loc : &str) -> io::Result<File>{
        let file = File::create(loc);
        return file;
    }
    
    pub fn read_lines(loc : &str) -> String{
        let mut res = String::new();
        let res_lines = create_line_iterator(loc);
        match res_lines {
            Ok(lines) => {
                for line in lines.flatten(){
                    if !res.is_empty() {
                        res = res + "\n";
                    }
                    res = res + line.as_str();
                }
            }
            Err(v) => {
                panic!("Could not read fille {{{}}}, error : {{{}}}",loc, v);
            }
        }

        return res;
    }
    
}