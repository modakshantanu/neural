use crate::matrix::{self, sgm, sgmprm};
use crate::dataset;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::convert::TryInto;


pub struct Network {
    pub w: Vec<matrix::Matrix>, // weights
    pub b: Vec<matrix::Matrix>, // biases
    pub layers: i32,
    pub layer_sizes: Vec<i32>,
}

impl Network {
    pub fn new(layer_sizes: &Vec<i32>) -> Network {
        let layers = layer_sizes.len();
        
        let mut weights: Vec<matrix::Matrix> = Vec::new();
        let mut biases: Vec<matrix::Matrix> = Vec::new();
        for i in 0usize..layers - 1 {
            let r = layer_sizes[i + 1];
            let c = layer_sizes[i];
            weights.push(matrix::Matrix::new(r, c));
            weights[i].randomize();
            biases.push(matrix::Matrix::new(r, 1));
            biases[i].randomize();   
        }

        return Network {w: weights, b: biases, layers: layers as i32, layer_sizes: layer_sizes.clone()};
    }

    pub fn feedfwd(&self, mut a: matrix::Matrix) -> matrix::Matrix {
        
        for i in 0..self.layers - 1 {
            // self.w[i as usize].assert_no_nan();
            // self.b[i as usize].assert_no_nan();
            // a.assert_no_nan();
            a = matrix::sgm(&(&(&self.w[i as usize] * &a) + &self.b[i as usize]));
        }
        return a;
    }

    pub fn print(&self) {
        for i in 0usize..self.layers as usize - 1 {
            let w = &self.w[i];
            let b = &self.b[i];
            println!("Layer {} weights", i);
            w.print();
            println!("Layer {} biases", i);
            b.print();

        }
    }


    pub fn train(&mut self, training: &Vec<dataset::Sample>, test: &Vec<dataset::Sample>, epochs: i32, mini_batch_size: usize, eta: f64) {
        let mut training = training.clone();
        let mut rng = thread_rng();

        for e in 0..epochs {
            training.shuffle(&mut rng);
            for i in (0..training.len()).step_by(mini_batch_size) {
                self.process_mini_batch(&training[i..i+mini_batch_size], eta);
            }
            let correct = self.selfeval(&test);
            println!("Epoch {}, {} / {}", e, correct, test.len());
        }
    }

    pub fn process_mini_batch(&mut self, mini_batch: &[dataset::Sample], eta: f64) {
        let mut grad_b: Vec<matrix::Matrix> = self.b.iter().map(|bm| matrix::Matrix::new(bm.rows, bm.cols)).collect();
        let mut grad_w: Vec<matrix::Matrix> = self.w.iter().map(|wm| matrix::Matrix::new(wm.rows, wm.cols)).collect();
        
        for sample in mini_batch {
            // Get the gradient of biases and weights for one sample
            let (del_b, del_w) = self.backprop(&sample);
            // Add to the running total of gradients 
            grad_b = grad_b.iter().zip(del_b.iter()).map(|(b, db)| b + db).collect();
            grad_w = grad_w.iter().zip(del_w.iter()).map(|(b, dw)| b + dw).collect();
            
        }

        // Update weights & biases for that mini batch
        self.w = self.w.iter().zip(grad_w.iter()).map(|(w, gw)| w - &gw.scale(eta/(mini_batch.len() as f64))).collect();
        self.b = self.b.iter().zip(grad_b.iter()).map(|(b, gb)| b - &gb.scale(eta/(mini_batch.len() as f64))).collect();
        
        
    }

    fn backprop(&self, sample: &dataset::Sample) -> (Vec<matrix::Matrix>, Vec<matrix::Matrix>) {
        let mut grad_b: Vec<matrix::Matrix> = self.b.iter().map(|bm| matrix::Matrix::new(bm.rows, bm.cols)).collect();
        let mut grad_w: Vec<matrix::Matrix> = self.w.iter().map(|wm| matrix::Matrix::new(wm.rows, wm.cols)).collect();
        let mut activation = sample.to_matrix();
        let wb_len = grad_b.len();
        let layers = self.layers as usize;
        

        let mut activations: Vec<matrix::Matrix> = Vec::new();
        activations.push(activation.clone());
        let mut zs: Vec<matrix::Matrix> = Vec::new();
        for i in 0..self.b.len() {
            let b = &self.b[i];
            let w = &self.w[i];            
            let z = &(w * &activation) + b;
            activation = sgm(&z);
            activations.push(activation.clone());
            zs.push(z);
        }
        
        // The change to activations
        // Initialize the del_z for the output layer based on the cost derivative * sigmoid' prime
        // del_z refers to how much the input must change (?)
        let mut del_z = Network::cost_derivative(activations.last().unwrap(), sample.label);
        del_z.hadamard(&matrix::sgmprm(zs.last().unwrap()));
        
        // The biases need to change proportional to del z 
        grad_b[wb_len - 1] = del_z.clone();
        
        // The weights need to change proportional to del_z * previous activation
        // direction deterimined by sign of del_z, magnitude proportional to del_z * prev activation
        grad_w[wb_len - 1] =  &del_z * &(activations[layers - 2].transpose());
        
        let mut z: matrix::Matrix;
        let mut sp: matrix::Matrix;
        // loop from second last layer down to input layer
        for l in (1..layers-1).rev() {
            z = zs[l-1].clone(); // inputs to layer l
            sp = sgmprm(&z); 
            // change in z for current layer = change for next layer * change in weights * sigmoid prime
            del_z = &self.w[l].transpose() * &del_z;
            del_z.hadamard(&sp);
            
            grad_b[l - 1] = del_z.clone(); // Same as above
            grad_w[l - 1] = &del_z * &activations[l-1].transpose();
        
        }


        return (grad_b, grad_w);
    }

    fn cost_derivative(act: &matrix::Matrix, label: u8) -> matrix::Matrix {
        let mut res = matrix::Matrix::new(act.rows, act.cols);
        for i in 0..10 {
            res.data[i][0] = 2.0*(act.data[i][0] - if i == label as usize {1.0} else {0.0});
        }
        return res;
    }

    // Return number of corect guesses
    pub fn selfeval(&self, test: &Vec<dataset::Sample>) -> i32 {
        let mut cnt = 0;
        for s in test {
            let input = s.to_matrix();
            let output = self.feedfwd(input);
            let mut max_idx = 1000;
            let mut max_val = 0.0;
            for i in 0..10 {
                if output.data[i][0] > max_val {
                    max_val = output.data[i][0];
                    max_idx = i;
                }
            }
            if max_idx as u8 == s.label {
                cnt += 1;
            }
        }
        return cnt;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        // First 4 bytes are the number of layers
        res.extend_from_slice(&self.layers.to_be_bytes());
        // All the sizes as a vector
        for l_size in &self.layer_sizes {
            res.extend_from_slice(&l_size.to_be_bytes());
        }
        
        for i in 0..self.layers as usize - 1 {
            res.extend_from_slice(&self.w[i].to_be_bytes());
            res.extend_from_slice(&self.b[i].to_be_bytes());
        }
        return res;
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Network {
        let mut index: usize = 0;
        let layers = i32::from_be_bytes(bytes[index..index + 4].try_into().unwrap());
        index += 4;
        let mut layer_sizes: Vec<i32> = Vec::new();
        for _ in 0..layers {
            layer_sizes.push(i32::from_be_bytes(bytes[index..index + 4].try_into().unwrap()));
            index += 4;
        }
        let mut w: Vec<matrix::Matrix> = Vec::new();
        let mut b: Vec<matrix::Matrix> = Vec::new();
        for _ in 0..layers - 1 {
            let (matrix, i) = matrix::Matrix::from_be_bytes(bytes, index);
            w.push(matrix);
            index = i;
            let (matrix, i) = matrix::Matrix::from_be_bytes(bytes, index);
            b.push(matrix);
            index = i;    
        }

        return Network {layers, layer_sizes, w, b};
    }

}