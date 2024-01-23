mod complex;
mod gate;
mod register;

use register::QuantumRegister;

fn main() {
    let mut register = QuantumRegister::<2>::new();
    let hadamard = gate::hadamard();

    dbg!(register);
    register.apply(&hadamard);
    dbg!(register);
}
