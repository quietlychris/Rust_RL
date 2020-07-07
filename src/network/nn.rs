use ndarray::{Array, Array1, Array2, Array3, ArrayD, Ix1};
use crate::network::layer::LayerType;
use crate::network::layer_trait::Layer;

pub struct HyperParameter {
  batch_size: usize,
  learning_rate: f32,
  _gamma: f32,
  _decay_rate: f32,
  _resume: bool,
  _render: bool,
}
impl HyperParameter {
  pub fn new() -> Self {
    HyperParameter{
      batch_size: 1,//10,//128,
      learning_rate: 0.002, //10e-4
      _gamma: 0.99,
      _decay_rate: 0.99,
      _resume: false,
      _render: false,
    }
  }
  pub fn batch_size(&mut self, batch_size: usize) {
    self.batch_size = batch_size;
  }
  pub fn learning_rate(&mut self, learning_rate: f32) {
    self.learning_rate = learning_rate;
  }
}


pub struct NeuralNetwork {
  input_dims: Vec<Vec<usize>>, //each layer takes a  1 to 4-dim input. Store details here
  h_p: HyperParameter,
  layers: Vec<LayerType>,
  last_input:  ArrayD<f32>,
  last_output: Array1<f32>,
  last_target: Array1<f32>,
  error: String,
}


impl NeuralNetwork {

  fn new(error: String) -> Self {
    // if error not in "bce" or "cce" => panic?
    NeuralNetwork{
      error,
      input_dims: vec![vec![]],
      layers:  vec![],
      h_p: HyperParameter::new(),
      last_input:  Array::zeros(0).into_dyn(),
      last_output: Array::zeros(0),
      last_target: Array::zeros(0),
    }
  }

  pub fn new1d(input_dim: usize, error: String) -> Self {
    let nn1 = NeuralNetwork::new(error);
    NeuralNetwork{
      input_dims: vec![vec![input_dim]],
      last_input: Array::zeros(input_dim).into_dyn(),
      ..nn1
    }
  }
  pub fn new2d((input_dim1, input_dim2): (usize, usize), error: String) -> Self {
    let nn1 = NeuralNetwork::new(error);
    NeuralNetwork{
      input_dims: vec![vec![input_dim1, input_dim2]],
      last_input: Array::zeros((input_dim1, input_dim2)).into_dyn(),
      ..nn1
    }
  }
  pub fn new3d((input_dim1,input_dim2,input_dim3): (usize,usize,usize), error: String) -> Self {
    let nn1 = NeuralNetwork::new(error);
    NeuralNetwork{
      input_dims: vec![vec![input_dim1, input_dim2, input_dim3]],
      last_input: Array::zeros((input_dim1, input_dim2, input_dim3)).into_dyn(),
      ..nn1
    }
  }

  pub fn set_batch_size(&mut self, batch_size: usize) {
    self.h_p.batch_size(batch_size);
  }
  pub fn set_learning_rate(&mut self, learning_rate: f32) {
    self.h_p.learning_rate(learning_rate);
  }

  pub fn add_activation(&mut self, layer_kind: &str) {
    let new_activation = LayerType::new_activation(layer_kind.to_string());
    match new_activation {
      Err(error) => {
        eprintln!("{}",error); 
        return;
      }
      Ok(activation) => {
        self.layers.push(activation);
        self.input_dims.push(self.input_dims.last().unwrap().clone()); // activation layers don't change dimensions
      }
    }
  }

  pub fn add_dense(&mut self, output_dim: usize) {
    if output_dim <= 0 {
      eprintln!("output dimension should be > 0! Doing nothing!");
      return;
    }
    let input_dims = self.input_dims.last().unwrap();
    if input_dims.len()>1 {
      eprintln!("Dense just accepts 1d input! Doing nothing!");
      return;
    }
    let dense_layer = LayerType::new_connection(input_dims[0], output_dim, self.h_p.batch_size, self.h_p.learning_rate);
    match dense_layer {
      Err(error) => {
        eprintln!("{}",error);
        return;
      }
      Ok(dense) => {
        self.layers.push(dense);
        self.input_dims.push(vec![output_dim]);
      }
    }
  }

  pub fn add_flatten(&mut self) {
    let input_dims = self.input_dims.last().unwrap();
    if input_dims.len()==1 {
      eprintln!("Input dimension is already one! Doing nothing!");
      return;
    }
    let flatten_layer = LayerType::new_flatten(input_dims.to_vec());
    match flatten_layer {
      Err(error) => {
        eprintln!("{}",error);
        return;
      }
      Ok(flatten) => {
        self.layers.push(flatten);
        let elements = input_dims.iter().fold(1, |prod, val| prod * val);
        self.input_dims.push(vec![elements]); 
      }
    }
  }

}

  

//https://gombru.github.io/2018/05/23/cross_entropy_loss/
//https://towardsdatascience.com/implementing-the-xor-gate-using-backpropagation-in-neural-networks-c1f255b4f20d
pub fn binary_crossentropy(target: Array1<f32>, output: Array1<f32>) -> Array1<f32> { //should be used after sigmoid
  //assert len of output vector = 1
  output-target
}

//https://stats.stackexchange.com/questions/235528/backpropagation-with-softmax-cross-entropy
pub fn categorical_crossentropy(target: Array1<f32>, output: Array1<f32>) -> Array1<f32> { //should be used after softmax
  -target / output
}

impl NeuralNetwork {

  pub fn print_setup(&self) {
    println!("printing neural network layers");
    for i in 0..self.layers.len() {
      println!("{}",self.layers[i].get_type());
    }
    println!();
  }

  pub fn forward1d(&mut self, x: Array1<f32>) -> Array1<f32> {
    self.forward(x.into_dyn())
  }
  pub fn forward2d(&mut self, x: Array2<f32>) -> Array1<f32> {
    self.forward(x.into_dyn())
  }
  pub fn forward3d(&mut self, x: Array3<f32>) -> Array1<f32> {
    self.forward(x.into_dyn())
  }


  pub fn forward(&mut self, x: ArrayD<f32>) -> Array1<f32> {
    self.last_input = x.clone();
    let mut input = x.into_dyn();//normalize(x);
    for i in 0..self.layers.len() {
      input = self.layers[i].forward(input);
    }
    self.last_output = input.into_dimensionality::<Ix1>().unwrap(); //output should be Array1 again
    self.last_output.clone()
  }


  pub fn backward(&mut self, target: Array1<f32>) {
    self.last_target = target.clone();
    match self.error.as_str() {
      "bce" => { //assert that target.len() == output.len() == 1
        let mut fb = binary_crossentropy(target, self.last_output.clone()).into_dyn(); 
        for i in (0..self.layers.len()).rev() {
          fb = self.layers[i].backward(fb);
        }
      },
      "cce" => {  
        let mut fb = categorical_crossentropy(target, self.last_output.clone()).into_dyn();
        for i in (0..self.layers.len()).rev() {
          fb = self.layers[i].backward(fb);
        }
      },
      _ => eprintln!("error, error function unknown! {}",self.error),
    }
  }

  pub fn error(&mut self, target: Array1<f32>) -> f32 {
    match self.error.as_str() {
      "bce" => {
        let t = target[0];
        let o = self.last_output[0];
        return -t*o.ln() - (1.-t)*(1.-o).ln();
      },
      "cce" => {
        return target.iter()
          .zip(self.last_output.iter())
          .fold(0.0, |sum, (&t, &o)| sum + t * -(o.ln()))
      },
      _ => eprintln!("foo"),
    }
    while 1==1 {println!("FOO");}
    42.
  }

}


//.map(|&x| if x < 0 { 0 } else { x }); //ReLu for multilayer

