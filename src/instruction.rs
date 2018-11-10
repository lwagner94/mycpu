pub enum Instruction {
    LoadImmediate(usize, u32),
    Increment(usize),
    Decrement(usize),
    Add(usize, usize, usize),
    Subtract(usize, usize, usize),
    Multiply(usize, usize, usize),
    Divide(usize, usize, usize),
    Or(usize, usize, usize),
    And(usize, usize, usize),
    XOr(usize, usize, usize),
    Negate(usize),
    Complement(usize)
}
