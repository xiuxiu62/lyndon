use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    #[inline]
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    #[inline]
    pub fn normalized_squared(&self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }

    #[inline]
    pub fn random(rng: &mut ThreadRng) -> Self {
        Self::new(rng.gen_range(0.0..1.0), 0.0)
    }

    #[inline]
    pub const fn real(&self) -> f64 {
        self.real
    }

    #[inline]
    pub const fn imaginary(&self) -> f64 {
        self.imaginary
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, other: Complex) -> Self::Output {
        let real = self.real * other.real - self.imaginary * other.imaginary;
        let imaginary = self.real * other.imaginary + self.imaginary * other.real;

        Self { real, imaginary }
    }
}
