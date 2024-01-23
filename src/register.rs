use crate::{complex::Complex, gate::Gate};

#[derive(Clone, Copy, Debug)]
pub struct QuantumRegister<const N: usize>([Complex; N]);

impl<const N: usize> QuantumRegister<N> {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self([Complex::random(&mut rng); N])
    }

    pub fn apply(&mut self, gate: &Gate<N>) {
        gate.apply(self);
    }
}

impl<const N: usize> AsRef<[Complex; N]> for QuantumRegister<N> {
    fn as_ref(&self) -> &[Complex; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[Complex; N]> for QuantumRegister<N> {
    fn as_mut(&mut self) -> &mut [Complex; N] {
        &mut self.0
    }
}
