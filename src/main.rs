//the neural network: input | first | second | out
//                     49       10      10      2

fn run_first_layer(weights: Vec<Vec<f64>>, biases: Vec<f64>, inputs: Vec<Vec<f64>>) -> Vec<f64> {
    let mut out_list: Vec<f64> = vec![];
    for i in 0..inputs.len() {
        let mut out: f64 = 0.0;
        for n in 0..inputs[i].len() {
            out += weights[i][n] * inputs[i][n];
        }
        out += biases[i];
        out_list.append(&mut vec![out]);
    }
    out_list
}

fn run_second_layer(weights: Vec<Vec<f64>>, biases: Vec<f64>, inputs: Vec<Vec<f64>>) -> Vec<f64> {
    let mut out_list: Vec<f64> = vec![];
    for i in 0..inputs.len() {
        let mut out: f64 = 0.0;
        for n in 0..inputs[i].len() {
            out += weights[i][n] * inputs[i][n];
        }
        out += biases[i];
        out_list.append(&mut vec![out]);
    }
    out_list
}

fn sigmoid(input: f64) -> f64{
    1.0 / (1.0 + f64::powf(2.71828, input * -1.0))
}

fn main() {
    
}
