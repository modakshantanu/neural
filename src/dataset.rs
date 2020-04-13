use std::fs;
use crate::matrix::Matrix;

#[derive(Clone)]
pub struct Sample {
    pub label: u8,
    pub data: [[f64; 28]; 28]
}

impl Sample {
    pub fn print(&self) {
        for row in &self.data {
            for i in row {
                if *i < 0.5 {
                    print!(".");
                } else {
                    print!("X");
                }
            }
            println!("");
        }
        println!("Label: {}", self.label);
    }

    pub fn to_matrix(&self) -> Matrix {
        let mut res = Matrix::new(784, 1);
        for i in 0..28 {
            for j in 0..28 {
                res.data[28 * i + j][0] = self.data[i][j];
            }
        }
        return res;
    }
}

pub fn get_dataset(imagepath: &str, labelpath: &str) -> Vec<Sample> {
    let mut res: Vec<Sample> = Vec::new();

    let img_bytes = fs::read(imagepath).unwrap();
    let label_bytes = fs::read(labelpath).unwrap();
    
    let mut magic = to_i32(&img_bytes[0..4]);
    assert!(magic == 2051); 
    magic = to_i32(&label_bytes[0..4]);
    assert!(magic == 2049);

    let num = to_i32(&img_bytes[4..8]);
    assert_eq!(num, to_i32(&label_bytes[4..8])); // check that they have same number of samples

    let mut label_idx = 8usize;
    let mut img_idx = 16usize;
    for _ in 0..num {
        
        let label = label_bytes[label_idx];
        label_idx += 1;        

        let mut data = [[0.0; 28]; 28];
        for i in 0..28 {
            for j in 0..28 {
                data[i][j] = (img_bytes[img_idx] as f64) / 255.0;
                img_idx += 1;
            }
        }
        res.push(Sample { label , data });
    }

    return res;
}

fn to_i32(slice: &[u8]) -> i32 {
    let mut res = 0i32;
    for i in slice {
        res <<= 8;
        res += *i as i32;
    }
    return res;
} 