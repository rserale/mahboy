pub struct Cpu {
    //Registers
    pub a: u8, //accumulator
    pub f: u8,//flag register, set if the previous operation fulfills certain conditions (see below) :
    /*
    7th bit: zero flag (Z), set if result == 0
    6: substraction flag (N), if a substraction happened
    5: half carry flag (H), carry between bit 3 and 4
    4: carry flag (C), carry in the heaviest bit of the operand
     */
    pub b: u8,//b to l are general purpose registers
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}

impl Cpu {
    /// Initialization
    pub fn new() -> Self {
        Cpu {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE, // Standard initialization of GB
            pc: 0x100,  // Programs start there
        }
    }

    pub fn read_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn write_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn read_flag_z(&self) -> bool {
        self.f & 0x80 != 0
    }

    pub fn read_flag_n(&self) -> bool {
        self.f & 0x40 != 0
    }

    pub fn read_flag_h(&self) -> bool {
        self.f & 0x20 != 0
    }

    pub fn read_flag_c(&self) -> bool {
        self.f & 0x10 != 0
    }

    pub fn set_flag_z(&mut self, value: bool) {
        if value {
            self.f |= 0x80;
        } else {
            self.f &= 0x7F;
        }
    }

    pub fn set_flag_n(&mut self, value: bool) {
        if value {
            self.f |= 0x40;
        } else {
            self.f &= 0xBF;
        }
    }

    pub fn set_flag_h(&mut self, value: bool) {
        if value {
            self.f |= 0x20;
        } else {
            self.f &= 0xDF;
        }
    }

    pub fn set_flag_c(&mut self, value: bool) {
        if value {
            self.f |= 0x10;
        } else {
            self.f &= 0xEF;
        }
    }

    //instructions
    pub fn add_a_b(&mut self) {
        let result = self.a.wrapping_add(self.b);

        //updating flags
        self.set_flag_z(result == 0);
        self.set_flag_n(false);
        self.set_flag_h((self.a & 0x0F) + (self.b & 0x0F) > 0x0F);
        self.set_flag_c(self.a as u16 + self.b as u16 > 0xFF);

        self.a = result;
    }
    //TODO: implement other instructions and move them to another file

}
