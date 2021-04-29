
#[derive(Debug, Copy, Clone)]
pub enum Reg8 {
    A, F, B, C, D, E, H, L
}

#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    BC, DE, HL, AF, SP
}

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    Z, NZ, C, NC, Always
}

