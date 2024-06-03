extern  crate flate2;


use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

fn main(){
    if args().len() != 3 {
        eprintln!("Usage: `source` `target` ");
        return;
    }
    // read the file 
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // output file
    let output = File::create(args().nth(2).unwrap()).unwrap();

    // encode the file content
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "source len {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    
    println!("Target {:?}", output.metadata().unwrap().len());
    println!("Elapsed {:?}", start.elapsed());
}