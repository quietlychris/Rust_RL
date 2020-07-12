use crate::network::nn::NeuralNetwork;
use ndarray::{array, Array2};
use rand::Rng;



pub fn new() -> NeuralNetwork {
  //let mut nn = NeuralNetwork::new1d(2,"bce".to_string());
  let mut nn = NeuralNetwork::new1d(2,"none".to_string());
  nn.set_batch_size(4);
  nn.set_learning_rate(0.0001);
  nn.add_dense(2); //Dense with 2 output neurons
  nn.add_activation("sigmoid"); //Sigmoid
  nn.add_dense(1); //Dense with 1 output neuron
  //nn.add_activation("sigmoid"); //Sigmoid
  nn
}



pub fn test(nn: &mut NeuralNetwork, input: &Array2<f32>, feedback: &Array2<f32>) {
    for i in 0..4 {
      println!("input: {}, feedback: {}", input.row(i),feedback.row(i));
      let pred = nn.predict1d(input.row(i).into_owned());
      println!("output: {}",pred);
    }
    println!();
}


fn train(nn: &mut NeuralNetwork, num_games: usize, input: &Array2<f32>, feedback: &Array2<f32>) {

    for _ in 0..num_games {
      let move_number = rand::thread_rng().gen_range(0, input.nrows()) as usize;
      let current_input = input.row(move_number).into_owned().clone();
      nn.train1d(current_input, feedback.row(move_number).into_owned());
    }
}

pub fn test_xor() {
  let input = array![[0.,0.],[0.,1.],[1.,0.],[1.,1.]]; // XOR
  let feedback = array![[0.],[1.],[1.],[0.]]; //First works good with 200k examples
  let mut nn = new();
  nn.print_setup();

  for _ in 0..10 {
    train(&mut nn, 1_000, &input, &feedback);
    test(&mut nn, &input, &feedback);
  }

}