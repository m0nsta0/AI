use rand::Rng;

//the neural network: input | first | second | out
//                     49       10      10      2

struct NeuralNetwork {
    layer1_weights: Vec<Vec<f64>>,
    layer1_biases: Vec<f64>,
    layer2_weights: Vec<Vec<f64>>,
    layer2_biases: Vec<f64>,
    layer3_weights: Vec<Vec<f64>>,
    layer3_biases: Vec<f64>,
}

fn run_first_layer(weights: Vec<Vec<f64>>, biases: Vec<f64>, inputs: Vec<f64>) -> Vec<f64> {
    let mut out_list: Vec<f64> = vec![];
    for i in 0..weights.len() {
        let mut out: f64 = 0.0;
        for n in 0..weights[i].len() {
            out += weights[i][n] * inputs[i];
        }
        out += biases[i];
        out_list.append(&mut vec![out]);
    }
    out_list
}
fn run_second_layer(weights: Vec<Vec<f64>>, biases: Vec<f64>, inputs: Vec<f64>) -> Vec<f64> {
    let mut out_list: Vec<f64> = vec![];
    for i in 0..weights.len() {
        let mut out: f64 = 0.0;
        for n in 0..weights[i].len() {
            out += weights[i][n] * inputs[i];
        }
        out += biases[i];
        out_list.append(&mut vec![out]);
    }
    out_list
}
fn run_third_layer(weights: Vec<Vec<f64>>, biases: Vec<f64>, inputs: Vec<f64>) -> Vec<f64> {
    let mut out_list: Vec<f64> = vec![];
    for i in 0..2 {
        let mut out: f64 = 0.0;
        for n in 0..weights[i].len() {
            out += weights[i][n] * inputs[i];
        }
        out += biases[i];
        out_list.append(&mut vec![out]);
    }
    out_list
}
fn run_full(nn: NeuralNetwork, input: Vec<f64>) -> Vec<f64> {
    let output1 = run_first_layer(NeuralNetwork.layer1_weights, NeuralNetwork.layer1_biases, input);
    vec![0.0]
}
fn sigmoid(input: f64) -> f64{
    1.0 / (1.0 + f64::powf(2.71828, input * -1.0))
}

fn main() {
    
}
