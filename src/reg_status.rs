#[derive(Debug)]
pub struct RegStatus {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl From<u8> for RegStatus {
    fn from(value: u8) -> Self {
        RegStatus{
            carry: (value & 1) != 0,
            zero: (value & (1 << 1)) != 0,
            interrupt_disable: (value & (1 << 2)) != 0,
            break_command: (value & (1 << 4)) != 0,
            overflow: (value & (1 << 6)) != 0,
            negative: (value & (1 << 7)) != 0,
        }
    }
}
