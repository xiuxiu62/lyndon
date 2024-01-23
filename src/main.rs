mod complex;
mod gate;
mod register;

use register::QuantumRegister;

fn main() {
    let mut register = QuantumRegister::<3>::new();
    let hadamard = gate::hadamard();
    let cnot = gate::cnot(0, 1);
    let toffoli = gate::toffoli(0, 1, 2);
    let pauli_x = gate::pauli_x();

    dbg!(register);
    register.apply(&hadamard);
    dbg!(register);
    register.apply(&cnot);
    register.apply(&pauli_x);
    dbg!(register);
    register.apply(&toffoli);
    dbg!(register);
}
