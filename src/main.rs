use std::env;
use std::fs::*;
use std::io::{Read, BufReader, BufRead, Seek};
use std::ops::Add;

struct CAT{
    path: String
}
impl CAT{
    pub fn new(filename: String) -> Self{
        CAT { path: filename }
    }
    pub fn read_file(&self){
        //read file
        let file = File::open(&self.path);
        let file = match file{
            Ok(file) => file,
            Err(e) => panic!("Error! {}",e)
        };
        let reader = BufReader::new(file);

        //display file
        for line in reader.lines(){
            println!("{}",line.unwrap());
        }
    }
}

fn main() {
    //get command line arguments
    let args: Vec<String> = env::args().collect();
    let clone = args.clone();
    let filename = parse(&clone);
    let cat = CAT::new(filename);
    cat.read_file();
}

fn parse(args: &[String]) -> String{
    let filename = &args[1];
    let filename = filename.to_owned();
    filename
}


