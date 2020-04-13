use std::vec::Vec;
use std::ops;
use std::convert::TryInto;

#[derive(Clone)]
pub struct Matrix {
    pub rows: i32,
    pub cols: i32,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(r: i32, c: i32) -> Matrix {
        let data: Vec<Vec<f64>> = vec![vec![0.0 ; c as usize] ; r as usize];
        return Matrix {
            rows: r,
            cols: c,
            data
        };
    }

    pub fn print(&self) {
        for row in &self.data {
            for e in row {
                print!("{:.2}\t", e);
            }
            println!("");
        }
    }

    pub fn randomize(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i as usize][j as usize] = rand::random::<f64>() * 2.0 - 1.0;
            }
        }
    }

    pub fn from_vec(arr: &Vec<f64>) -> Matrix {
        let mut res = Matrix::new(arr.len() as i32,  1);
        for i in 0..arr.len() {
            res.data[i][0] = arr[i];
        }
        return res;
    }

    pub fn hadamard(&mut self, other: &Matrix) {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Hadamard: Dimension error");
        }

        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                self.data[i][j] *= other.data[i][j];
            }
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut res = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                res.data[j][i] = self.data[i][j];
            }
        }
        return res;
    }

    pub fn scale(&self, sc: f64) -> Matrix {
        let mut res = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                res.data[i][j] = self.data[i][j] * sc;
            }
        }
        return res;
    }

    pub fn assert_no_nan(&self) {
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                assert!(!self.data[i][j].is_nan());
            }
        }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.extend_from_slice(&self.rows.to_be_bytes());
        res.extend_from_slice(&self.cols.to_be_bytes());
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                res.extend_from_slice(&self.data[i][j].to_be_bytes());
            }
        }
        return res;
    }

    pub fn from_be_bytes(bytes: &[u8], mut index: usize) -> (Matrix, usize) {
        let rows = i32::from_be_bytes(bytes[index..index+4].try_into().unwrap());
        index += 4;
        let cols = i32::from_be_bytes(bytes[index..index+4].try_into().unwrap());
        index += 4;

        let mut data: Vec<Vec<f64>> = vec![vec![0.0 ; cols as usize] ; rows as usize];

        for i in 0..rows as usize {
            for j in 0..cols as usize {
                data[i][j] = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
                index += 8;
            }
        }
        return (Matrix {rows, cols, data}, index);
    }
    
}

impl ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Add: Different number of rows and columns!");
        } 
        let mut res = Matrix::new(self.rows, self.cols);
        for i in 0usize..self.rows as usize {
            for j in 0usize..self.cols as usize {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        return res;
    }
}

impl ops::Sub<&Matrix> for &Matrix {
    type Output = Matrix;
    fn sub(self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Sub: Different number of rows and columns!");
        } 
        let mut res = Matrix::new(self.rows, self.cols);
        for i in 0usize..self.rows as usize {
            for j in 0usize..self.cols as usize {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        return res;
    }
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Mul: Mismatched dimensions");
        }
        let mut res = Matrix::new(self.rows, other.cols);
        for i in 0usize..self.rows as usize {
            for j in 0usize..other.cols as usize {
                let mut sum = 0.0;
                for k in 0usize..self.cols as usize {
                    sum += self.data[i][k]*other.data[k][j];
                }
                res.data[i][j] = sum;
            }
        }
        return res;
    }
}


pub fn sgm(m: &Matrix) -> Matrix {
    let mut res = Matrix::new(m.rows, m.cols);
    for i in 0usize..m.rows as usize {
        for j in 0usize..m.cols as usize {
            res.data[i][j] = sigmoid(m.data[i][j]);
        }
    }
    return res;
}

pub fn sgmprm(m: &Matrix) -> Matrix {
    let mut res = Matrix::new(m.rows, m.cols);
    for i in 0usize..m.rows as usize {
        for j in 0usize..m.cols as usize {
            res.data[i][j] = sigprime(m.data[i][j]);
        }
    }
    return res;

}

fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp())
}

fn sigprime(x: f64) -> f64 {
    return sigmoid(x) * (1.0 - sigmoid(x));
}