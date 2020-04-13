mod matrix;
mod net;
mod dataset;
use std::io;
use std::fs;
use std::path::Path;


static TRAIN_IMAGES: &str = "train-images.idx3-ubyte";
static TRAIN_LABELS: &str = "train-labels.idx1-ubyte";
static TEST_IMAGES: &str = "t10k-images.idx3-ubyte";
static TEST_LABELS: &str = "t10k-labels.idx1-ubyte";


fn main() {

    let training = dataset::get_dataset(TRAIN_IMAGES, TRAIN_LABELS);
    let testing = dataset::get_dataset(TEST_IMAGES, TEST_LABELS);

    let mut filename = String::new();
    println!("Enter filename of network config");
    io::stdin().read_line(&mut filename).unwrap();
    filename = filename.trim().to_string();
    let mut nn: net::Network;
        
    if !filename.is_empty() {
        if !filename.ends_with(".ncf") {
            filename += ".ncf";
        }
        filename = format!("./{}",filename);
        let res = fs::read(&filename).unwrap();
        nn = net::Network::from_be_bytes(&res);
        
        let correct = nn.selfeval(&testing);
        println!("{} / {}", correct, testing.len());

    } else {
        nn = net::Network::new(&vec![784,200,10]);
        nn.train(&training, &testing, 10, 10, 1.5);
    }
    
    println!("Enter filename to save network");
    io::stdin().read_line(&mut filename).unwrap();
    filename = filename.trim().to_string();
    
    if !filename.is_empty() {
        if !filename.ends_with(".ncf") {
            filename += ".ncf";
        }
        filename = format!("./{}",filename);
        let bytes = nn.to_bytes();
        fs::write(filename, bytes).unwrap();
    } 


}
