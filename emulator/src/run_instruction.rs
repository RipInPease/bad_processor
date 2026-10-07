use super::{CPU, Instruction, AddressingMode};

impl CPU {
    pub fn run_instruction(&mut self) {

    }

    /// Decodes the next instruction
    fn decode(&mut self) -> Option<(Instruction, AddressingMode)> {
        use Instruction::*;
        use AddressingMode::*;

        let function_code = self.ram.bytes[self.registers.program_counter as usize];
        self.registers.increment_pc();

        match function_code {
            // ADC - Add with carry
            0x69 => Some((ADC, Immediate)),
            0x65 => Some((ADC, ZeroPage)),
            0x75 => Some((ADC, ZeroPageX)),
            0x6D => Some((ADC, Absolute)),
            0x7D => Some((ADC, AbsoluteX)),
            0x79 => Some((ADC, AbsoluteY)),
            0x61 => Some((ADC, IndirectX)),
            0x71 => Some((ADC, IndirectY)),

            // AND - Logical AND
            0x29 => Some((AND, Immediate)),
            0x25 => Some((AND, ZeroPage)),
            0x35 => Some((AND, ZeroPageX)),
            0x2D => Some((AND, Absolute)),
            0x3D => Some((AND, AbsoluteX)),
            0x39 => Some((AND, AbsoluteY)),
            0x21 => Some((AND, IndirectX)),
            0x31 => Some((AND, IndirectY)),

            // ASL - Arithmetic shift left
            0x0A => Some((ASL, Accumulator)),
            0x06 => Some((ASL, ZeroPage)),
            0x16 => Some((ASL, ZeroPageX)),
            0x0E => Some((ASL, Absolute)),
            0x1E => Some((ASL, AbsoluteX)),

            // BCC - Branch if carry clear
            0x90 => Some((BCC, Relative)),

            // BCS - Branch if carry set
            0xB0 => Some((BCS, Relative)),

            // BEQ - Branch if equal
            0xF0 => Some((BEQ, Relative)),

            // BIT - Test bits
            0x24 => Some((BIT, ZeroPage)),
            0x2C => Some((BIT, Absolute)),

            // BMI - Branch if minus
            0x30 => Some((BMI, Relative)),

            // BNE - Branch if not equal
            0xD0 => Some((BNE, Relative)),

            // BPL - Branch if plus
            0x10 => Some((BPL, Relative)),

            // BRK - Force interrupt
            0x00 => Some((BRK, Implied)),

            // BVC - Branch if overflow clear
            0x50 => Some((BVC, Relative)),

            // BVS - Branch if overflow set
            0x70 => Some((BVS, Relative)),

            // CLC - Clear carry
            0x18 => Some((CLC, Implied)),

            // CLD - Clear decimal
            0xD8 => Some((CLD, Implied)),

            // CLI - Clear interrupt disable
            0x58 => Some((CLI, Implied)),

            // CLV - Clear overflow
            0xB8 => Some((CLV, Implied)),

            // CMP - Compare accumulator
            0xC9 => Some((CMP, Immediate)),
            0xC5 => Some((CMP, ZeroPage)),
            0xD5 => Some((CMP, ZeroPageX)),
            0xCD => Some((CMP, Absolute)),
            0xDD => Some((CMP, AbsoluteX)),
            0xD9 => Some((CMP, AbsoluteY)),
            0xC1 => Some((CMP, IndirectX)),
            0xD1 => Some((CMP, IndirectY)),

            // CPX - Compare X
            0xE0 => Some((CPX, Immediate)),
            0xE4 => Some((CPX, ZeroPage)),
            0xEC => Some((CPX, Absolute)),

            // CPY - Compare Y
            0xC0 => Some((CPY, Immediate)),
            0xC4 => Some((CPY, ZeroPage)),
            0xCC => Some((CPY, Absolute)),

            // DEC - Decrement memory
            0xC6 => Some((DEC, ZeroPage)),
            0xD6 => Some((DEC, ZeroPageX)),
            0xCE => Some((DEC, Absolute)),
            0xDE => Some((DEC, AbsoluteX)),

            // DEX - Decrement X
            0xCA => Some((DEX, Implied)),

            // DEY - Decrement Y
            0x88 => Some((DEY, Implied)),

            // EOR - Exclusive OR
            0x49 => Some((EOR, Immediate)),
            0x45 => Some((EOR, ZeroPage)),
            0x55 => Some((EOR, ZeroPageX)),
            0x4D => Some((EOR, Absolute)),
            0x5D => Some((EOR, AbsoluteX)),
            0x59 => Some((EOR, AbsoluteY)),
            0x41 => Some((EOR, IndirectX)),
            0x51 => Some((EOR, IndirectY)),

            // INC - Increment memory
            0xE6 => Some((INC, ZeroPage)),
            0xF6 => Some((INC, ZeroPageX)),
            0xEE => Some((INC, Absolute)),
            0xFE => Some((INC, AbsoluteX)),

            // INX - Increment X
            0xE8 => Some((INX, Implied)),

            // INY - Increment Y
            0xC8 => Some((INY, Implied)),

            // JMP - Jump
            0x4C => Some((JMP, Absolute)),
            0x6C => Some((JMP, Indirect)),

            // JSR - Jump to subroutine
            0x20 => Some((JSR, Absolute)),

            // LDA - Load accumulator
            0xA9 => Some((LDA, Immediate)),
            0xA5 => Some((LDA, ZeroPage)),
            0xB5 => Some((LDA, ZeroPageX)),
            0xAD => Some((LDA, Absolute)),
            0xBD => Some((LDA, AbsoluteX)),
            0xB9 => Some((LDA, AbsoluteY)),
            0xA1 => Some((LDA, IndirectX)),
            0xB1 => Some((LDA, IndirectY)),

            // LDX - Load X
            0xA2 => Some((LDX, Immediate)),
            0xA6 => Some((LDX, ZeroPage)),
            0xB6 => Some((LDX, ZeroPageY)),
            0xAE => Some((LDX, Absolute)),
            0xBE => Some((LDX, AbsoluteY)),

            // LDY - Load Y
            0xA0 => Some((LDY, Immediate)),
            0xA4 => Some((LDY, ZeroPage)),
            0xB4 => Some((LDY, ZeroPageX)),
            0xAC => Some((LDY, Absolute)),
            0xBC => Some((LDY, AbsoluteX)),

            // LSR - Logical shift right
            0x4A => Some((LSR, Accumulator)),
            0x46 => Some((LSR, ZeroPage)),
            0x56 => Some((LSR, ZeroPageX)),
            0x4E => Some((LSR, Absolute)),
            0x5E => Some((LSR, AbsoluteX)),

            // NOP - No operation
            0xEA => Some((NOP, Implied)),

            // ORA - Logical OR
            0x09 => Some((ORA, Immediate)),
            0x05 => Some((ORA, ZeroPage)),
            0x15 => Some((ORA, ZeroPageX)),
            0x0D => Some((ORA, Absolute)),
            0x1D => Some((ORA, AbsoluteX)),
            0x19 => Some((ORA, AbsoluteY)),
            0x01 => Some((ORA, IndirectX)),
            0x11 => Some((ORA, IndirectY)),

            // PHA - Push accumulator
            0x48 => Some((PHA, Implied)),

            // PHP - Push processor status
            0x08 => Some((PHP, Implied)),

            // PLA - Pull accumulator
            0x68 => Some((PLA, Implied)),

            // PLP - Pull processor status
            0x28 => Some((PLP, Implied)),

            // ROL - Rotate left
            0x2A => Some((ROL, Accumulator)),
            0x26 => Some((ROL, ZeroPage)),
            0x36 => Some((ROL, ZeroPageX)),
            0x2E => Some((ROL, Absolute)),
            0x3E => Some((ROL, AbsoluteX)),

            // ROR - Rotate right
            0x6A => Some((ROR, Accumulator)),
            0x66 => Some((ROR, ZeroPage)),
            0x76 => Some((ROR, ZeroPageX)),
            0x6E => Some((ROR, Absolute)),
            0x7E => Some((ROR, AbsoluteX)),

            // RTI - Return from interrupt
            0x40 => Some((RTI, Implied)),

            // RTS - Return from subroutine
            0x60 => Some((RTS, Implied)),

            // SBC - Subtract with carry
            0xE9 => Some((SBC, Immediate)),
            0xE5 => Some((SBC, ZeroPage)),
            0xF5 => Some((SBC, ZeroPageX)),
            0xED => Some((SBC, Absolute)),
            0xFD => Some((SBC, AbsoluteX)),
            0xF9 => Some((SBC, AbsoluteY)),
            0xE1 => Some((SBC, IndirectX)),
            0xF1 => Some((SBC, IndirectY)),

            // SEC - Set carry
            0x38 => Some((SEC, Implied)),

            // SED - Set decimal
            0xF8 => Some((SED, Implied)),

            // SEI - Set interrupt disable
            0x78 => Some((SEI, Implied)),

            // STA - Store accumulator
            0x85 => Some((STA, ZeroPage)),
            0x95 => Some((STA, ZeroPageX)),
            0x8D => Some((STA, Absolute)),
            0x9D => Some((STA, AbsoluteX)),
            0x99 => Some((STA, AbsoluteY)),
            0x81 => Some((STA, IndirectX)),
            0x91 => Some((STA, IndirectY)),

            // STX - Store X
            0x86 => Some((STX, ZeroPage)),
            0x96 => Some((STX, ZeroPageY)),
            0x8E => Some((STX, Absolute)),

            // STY - Store Y
            0x84 => Some((STY, ZeroPage)),
            0x94 => Some((STY, ZeroPageX)),
            0x8C => Some((STY, Absolute)),

            // TAX - Transfer A to X
            0xAA => Some((TAX, Implied)),

            // TAY - Transfer A to Y
            0xA8 => Some((TAY, Implied)),

            // TSX - Transfer stack pointer to X
            0xBA => Some((TSX, Implied)),

            // TXA - Transfer X to A
            0x8A => Some((TXA, Implied)),

            // TXS - Transfer X to stack pointer
            0x9A => Some((TXS, Implied)),

            // TYA - Transfer Y to A
            0x98 => Some((TYA, Implied)),

            _ => None,
        }
    }

    /// Add with carry
    fn adc(&mut self, rhs: u8) {
        // Check overflow flag
        // This is a shitty implementation of i8::overflowing_add 
        // because it is an experimental feature
        let (temp, a) = (self.registers.accumulator as i8).overflowing_add(rhs as i8);
        let (_, b) = temp.overflowing_add(self.registers.carry_flag() as i8);
        
        if a != b {
            self.registers.set_overflow_flag();
        } else {
            self.registers.reset_overflow_flag();
        }

        // Perform the operation
        let (res, carry) =  self.registers.accumulator.carrying_add(
            rhs,
            self.registers.carry_flag()
        );
        self.registers.accumulator = res;


        // Check other flags
        if carry {
            self.registers.set_carry_flag();
        } else {
            self.registers.reset_carry_flag();
        }

        if self.registers.accumulator == 0 {
            self.registers.set_zero_flag();
        } else {
            self.registers.reset_zero_flag();
        }

        if self.registers.accumulator >> 7 & 1 == 1 {
            self.registers.set_negative_flag();
        } else {
            self.registers.reset_negative_flag();
        }
    }

    /// Logical AND
    pub fn and(&mut self, rhs: u8) {
        self.registers.accumulator &= rhs;

        if self.registers.accumulator == 0 {
            self.registers.set_zero_flag();
        } else {
            self.registers.reset_zero_flag();
        }

        if self.registers.accumulator >> 7 & 1 == 1 {
            self.registers.set_negative_flag();
        } else {
            self.registers.reset_negative_flag();
        }
    }

    /// Arithmetic left shift
    pub fn asl(&mut self, rhs: u8) {
        // Move bit 7 to carry flag
        if self.registers.accumulator >> 7 & 1 == 1 {
            self.registers.set_carry_flag();
        } else {
            self.registers.reset_carry_flag();
        }

        // Perform operation
        self.registers.accumulator <<= rhs;

        // Set flags
        if self.registers.accumulator == 0 {
            self.registers.set_zero_flag();
        } else {
            self.registers.reset_zero_flag();
        }

        if self.registers.accumulator >> 7 & 1 == 1 {
            self.registers.set_negative_flag();
        } else {
            self.registers.reset_negative_flag();
        }
    }

    /// Branch if carry clear
    pub fn bcc(&mut self, displacement: i8) {
        if self.registers.carry_flag() {
            return
        }

        if displacement < 0 {
            self.registers.program_counter -= (displacement as i16).abs() as u16;
        } else {
            self.registers.program_counter += (displacement as i16) as u16;
        }
    }

    /// Branch if carry set
    pub fn bcs(&mut self, displacement: i8) {
        if !self.registers.carry_flag() {
            return
        }

        if displacement < 0 {
            self.registers.program_counter -= (displacement as i16).abs() as u16;
        } else {
            self.registers.program_counter += (displacement as i16) as u16;
        }
    }

    /// Branch if zero flag set
    pub fn beq(&mut self, displacement: i8) {
        if !self.registers.zero_flag() {
            return
        }

        if displacement < 0 {
            self.registers.program_counter -= (displacement as i16).abs() as u16;
        } else {
            self.registers.program_counter += (displacement as i16) as u16;
        }
    }

    /// Bit test
    pub fn bit(&mut self, rhs: u8) {
        let res = self.registers.accumulator & rhs;

        // Flags
        if res == 0 {
            self.registers.set_zero_flag();
        }else {
            self.registers.reset_zero_flag();
        }

        if res >> 6 & 1 == 1 {
            self.registers.set_overflow_flag();
        } else {
            self.registers.reset_overflow_flag();
        }

        if res >> 7 & 1 == 1 {
            self.registers.set_negative_flag();
        } else {
            self.registers.reset_negative_flag();
        }
    }

    /// Branch if not equal
    pub fn bne(&mut self, displacement: i8) {
        if self.registers.zero_flag() {
            return
        }

        if displacement < 0 {
            self.registers.program_counter -= (displacement as i16).abs() as u16;
        } else {
            self.registers.program_counter += (displacement as i16) as u16;
        }
    }

    /// Branch if positve
    pub fn bpl(&mut self, displacement: i8) {
        if self.registers.negative_flag() {
            return
        }

        if displacement < 0 {
            self.registers.program_counter -= (displacement as i16).abs() as u16;
        } else {
            self.registers.program_counter += (displacement as i16) as u16;
        }
    }

    /// Force interrupt
    pub fn brk(&mut self) {
        todo!()
    }
}