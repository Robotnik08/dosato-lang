pub enum OpCode {
    Nop,
    Print(u8),
    Load(u16, u8),
    Store(u16, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Nop => write!(f, "Nop"),
            OpCode::Print(reg) => write!(f, "Print r{}", reg),
            OpCode::Load(addr, reg) => write!(f, "Load %{} -> r{}", addr, reg),
            OpCode::Store(addr, reg) => write!(f, "Store r{} -> %{}", reg, addr),
            OpCode::Add(dest, src1, src2) => write!(f, "Add r{}, r{}, r{}", dest, src1, src2),
            OpCode::Sub(dest, src1, src2) => write!(f, "Sub r{}, r{}, r{}", dest, src1, src2),
        }
    }
}