const INPUTS: usize = 4;
const HIDDEN: usize = 6;
const OUTPUTS: usize = 3;

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    pub weights_ih: [[f32; INPUTS]; HIDDEN],
    pub biases_h: [f32; HIDDEN],
    pub weights_ho: [[f32; HIDDEN]; OUTPUTS],
    pub biases_o: [f32; OUTPUTS],
}

impl NeuralNetwork {
    pub fn forward(&self, inputs: [f32; INPUTS]) -> [f32; OUTPUTS] {
        let mut hidden = [0.0; HIDDEN];

        for (i, neuron) in hidden.iter_mut().enumerate() {
            let mut sum = self.biases_h[i];

            for (j, input) in inputs.iter().enumerate() {
                sum += self.weights_ih[i][j] * input;
            }

            *neuron = sum.tanh();
        }

        let mut outputs = [0.0; OUTPUTS];

        for (i, output) in outputs.iter_mut().enumerate() {
            let mut sum = self.biases_o[i];

            for (j, neuron) in hidden.iter().enumerate() {
                sum += self.weights_ho[i][j] * neuron;
            }

            *output = sum.tanh();
        }

        outputs
    }

    // Temporary reference brain that reproduces our
    // existing oscillating locomotion strategy.
    pub fn baseline() -> Self {
        let mut net = Self {
            weights_ih: [[0.0; INPUTS]; HIDDEN],
            biases_h: [0.0; HIDDEN],
            weights_ho: [[0.0; HIDDEN]; OUTPUTS],
            biases_o: [0.0; OUTPUTS],
        };

        // Input 2 = sin(phase)
        // Input 3 = cos(phase)
        net.weights_ih[0][2] = 2.0;
        net.weights_ih[1][3] = 2.0;
        net.weights_ih[2][3] = -2.0;

        // Output 0 = muscle command
        // Output 1 = left grip
        // Output 2 = right grip
        net.weights_ho[0][0] = 2.0;
        net.weights_ho[1][1] = 3.0;
        net.weights_ho[2][2] = 3.0;

        net
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_phases_swap_grip() {
        let net = NeuralNetwork::baseline();

        let a = net.forward([0.0, 0.0, 0.0, 1.0]);
        let b = net.forward([0.0, 0.0, 0.0, -1.0]);

        assert!(a[1] > a[2]);
        assert!(b[2] > b[1]);
    }
}
