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

pub fn pauli_x<const N: usize>() -> Gate<N> {
    Gate::new(
        (0..N)
            .map(|i| {
                (0..N)
                    .map(|j| Complex::new(if i == j { 0.0 } else { 1.0 }, 0.0))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    )
}

pub fn pauli_y<const N: usize>() -> Gate<N> {
    Gate::new(
        (0..N)
            .map(|i| {
                (0..N)
                    .map(|j| {
                        let sign = match i {
                            value if value == j => Complex::new(0.0, 1.0),
                            _ => Complex::new(0.0, -1.0),
                        };

                        sign * Complex::new(0.5, 0.0)
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

pub fn pauli_z<const N: usize>() -> Gate<N> {
    Gate::new(
        (0..N)
            .map(|i| {
                (0..N)
                    .map(|j| {
                        let sign = if i == j { 1.0 } else { -1.0 };

                        Complex::new(sign, 0.0)
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

pub fn cnot<const N: usize>(control: usize, target: usize) -> Gate<N> {
    Gate::new(
        (0..N)
            .map(|i| {
                (0..N)
                    .map(|j| match i {
                        _ if i == j => Complex::new(1.0, 0.0),
                        _ if i == control && i == target => Complex::new(0.0, 1.0),
                        _ => Complex::new(0.0, 0.0),
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

pub fn swap<const N: usize>(lhs: usize, rhs: usize) -> Gate<N> {
    let matrix = (0..N)
        .map(|i| {
            (0..N)
                .map(|j| match i {
                    value if value == j => Complex::new(1.0, 0.0),
                    _ if (i == lhs && j == rhs) || (i == rhs && j == lhs) => Complex::new(0.0, 1.0),
                    _ => Complex::new(0.0, 0.0),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Gate::new(matrix)
}

pub fn toffoli<const N: usize>(control_1: usize, control_2: usize, target: usize) -> Gate<N> {
    Gate::new(
        (0..N)
            .map(|i| {
                (0..N)
                    .map(|j| match i {
                        _ if i == j => Complex::new(1.0, 0.0),
                        _ if i == target && j == target => Complex::new(0.0, 1.0),
                        _ if (i == control_1 && j == control_1)
                            || (i == control_2 && j == control_2) =>
                        {
                            Complex::new(0.0, 1.0)
                        }
                        _ => Complex::new(0.0, 0.0),
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
