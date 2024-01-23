use crate::{complex::Complex, register::QuantumRegister};

pub struct Gate<const WIDTH: usize>([[Complex; WIDTH]; WIDTH]);

impl<const WIDTH: usize> Gate<WIDTH> {
    #[inline]
    pub const fn new(inner: [[Complex; WIDTH]; WIDTH]) -> Self {
        Self(inner)
    }

    #[inline]
    pub fn apply(&self, register: &mut QuantumRegister<WIDTH>) {
        let result = (0..WIDTH)
            .map(|i| {
                (0..WIDTH).fold(Complex::new(0.0, 0.0), |acc, j| {
                    let prod = self.0[i][j] * register.as_ref()[j];
                    Complex::new(acc.real() + prod.real(), acc.imaginary() + prod.imaginary())
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        *register.as_mut() = result;
    }
}

pub fn hadamard<const N: usize>() -> Gate<N> {
    let scale_factor = 1.0 / (N as f64).sqrt();

    Gate::new(
        (0..N)
            .map(|i| {
                (0..N)
                    .map(|j| {
                        Complex::new(
                            scale_factor * (-1.0_f64.powf((i as f64 + j as f64) / N as f64)),
                            0.0,
                        )
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    )
}
