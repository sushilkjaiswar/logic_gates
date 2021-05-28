use logic_gates::{and, xor};

pub type Sum = u8;
pub type Carry = u8;


pub fn half_adder_inout_output() -> Vec<((u8, u8), (Sum, Carry))> {
        vec![
            ((1,0),(0,1)),
            ((1,1),(1,0)),
            ((0,0),(0,0)),
            ((0,1),(0,1))
        ]
}

pub fn half_adder(a:u8, b:u8) -> (Sum, Carry) {
    (and(a, b), xor(a, b))
}

#[test]
fn one_bit_adder() {
        for (inn, out) in half_adder_inout_output() {
            let (a,b) = inn;
            println!("Input {}, {} -> {:?}", a, b, out);
            assert_eq!(half_adder(a, b), out);
        }
}
