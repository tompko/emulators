#[derive(Debug, Clone)]
pub struct RegStatus {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub break_command: bool,
    expansion: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl From<u8> for RegStatus {
    fn from(value: u8) -> Self {
        RegStatus{
            carry: (value & 1) != 0,
            zero: (value & (1 << 1)) != 0,
            interrupt_disable: (value & (1 << 2)) != 0,
            decimal: (value & (1 << 3)) != 0,
            break_command: (value & (1 << 4)) != 0,
            expansion: true,
            overflow: (value & (1 << 6)) != 0,
            negative: (value & (1 << 7)) != 0,
        }
    }
}

impl Into<u8> for RegStatus {
    fn into(self) -> u8 {
        let mut ret = 0;
        if self.carry {
            ret |= 1;
        }
        if self.zero {
            ret |= 1 << 1;
        }
        if self.interrupt_disable {
            ret |= 1 << 2;
        }
        if self.decimal {
            ret |= 1 << 3;
        }
        if self.break_command {
            ret |= 1 << 4;
        }
        if self.expansion {
            ret |= 1 << 5;
        }
        if self.overflow {
            ret |= 1 << 6;
        }
        if self.negative {
            ret |= 1 << 7;
        }
        ret
    }
}
