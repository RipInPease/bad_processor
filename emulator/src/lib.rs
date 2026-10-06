mod run_instruction;

struct CPU {
    registers: Registers,
    ram: RAM,
    state: ExecutionState,
}


/// 64 KiB of memory stored in little endian
struct RAM {
    bytes: [u8; 1024 * 64],
}    


pub struct Registers {
    program_counter: u16,

    /// References data in memory 0x0100 to 0x01FF
    stack_pointer: u8,

    /// Used in arithamtic operations
    accumulator: u8,

    /// Hold counters or offsets for accessing memory
    register_x: u8,

    /// Hold counters or offsets for accessing memory. Supports increments and decrements
    register_y: u8,

    /// Result of last operation
    process_status: u8,
}

impl Registers {
    pub const CARRY_FLAG_BIT_POSITION: u8 = 0;
    pub const ZERO_FLAG_BIT_POSITION: u8 = 1;
    pub const INTERRUPT_DISABLE_FLAG_BIT_POSITION: u8 = 2;
    pub const DECIMAL_MODE_FLAG_BIT_POSITION: u8 = 3;
    pub const BREAK_FLAG_BIT_POSITION: u8 = 4;
    pub const OVERFLOW_FLAG_BIT_POSITION: u8 = 6;
    pub const NEGATIVE_FLAG_BIT_POSITION: u8 = 7;


    /// Increments the program counter by 1
    pub fn increment_pc(&mut self) {
        self.program_counter += 1;
    }

    /// Reset all status flags from process_status register
    pub fn clear_flags(&mut self) {
        self.process_status = 0;
    }

    /// Set the carry flag
    pub fn set_carry_flag(&mut self) {
        self.process_status |= 1 << Self::CARRY_FLAG_BIT_POSITION;
    }

    /// Reset the carry flag
    pub fn reset_carry_flag(&mut self) {
        self.process_status &= !(1 << Self::CARRY_FLAG_BIT_POSITION)
    }
    
    /// Set the zero flag
    pub fn set_zero_flag(&mut self) {
        self.process_status |= 1 << Self::ZERO_FLAG_BIT_POSITION;
    }

    /// Reset the zero flag
    pub fn reset_zero_flag(&mut self) {
        self.process_status &= !(1 << Self::ZERO_FLAG_BIT_POSITION);
    }

    /// Set the interrupt disable flag
    pub fn set_interrupt_disable_flag(&mut self) {
        self.process_status |= 1 << Self::INTERRUPT_DISABLE_FLAG_BIT_POSITION;
    }

    /// Reset the interrupt disable flag
    pub fn reset_interrupt_disable_flag(&mut self) {
        self.process_status &= !(1 << Self::INTERRUPT_DISABLE_FLAG_BIT_POSITION);
    }

    /// Set the decimal mode flag
    pub fn set_decimal_mode_flag(&mut self) {
        self.process_status |= 1 << Self::DECIMAL_MODE_FLAG_BIT_POSITION;
    }

    /// Reset the decimal mode flag
    pub fn reset_decimal_mode_flag(&mut self) {
        self.process_status &= !(1 << Self::DECIMAL_MODE_FLAG_BIT_POSITION);
    }

    /// Set the break flag
    pub fn set_break_flag(&mut self) {
        self.process_status |= 1 << Self::BREAK_FLAG_BIT_POSITION;
    }

    /// Reset the break flag
    pub fn reset_break_flag(&mut self) {
        self.process_status &= !(1 << Self::BREAK_FLAG_BIT_POSITION);
    }

    /// Set the overflow flag
    pub fn set_overflow_flag(&mut self) {
        self.process_status |= 1 << Self::OVERFLOW_FLAG_BIT_POSITION;
    }

    /// Reset the overflow flag
    pub fn reset_overflow_flag(&mut self) {
        self.process_status &= !(1 << Self::OVERFLOW_FLAG_BIT_POSITION);
    }

    /// Set the negative flag
    pub fn set_negative_flag(&mut self) {
        self.process_status |= 1 << Self::NEGATIVE_FLAG_BIT_POSITION;
    }

    /// Reset the negative flag
    pub fn reset_negative_flag(&mut self) {
        self.process_status &= !(1 << Self::NEGATIVE_FLAG_BIT_POSITION);
    }
}


/// Where in memory to read or write for a certain [`Instruction`]
pub enum AddressingMode {
    /// If the addressing mode is implied for the instruction
    Implied,

    /// Work directly on the accumulator
    Accumulator,

    /// Immediate addressing allows the programmer to directly specify an 
    /// 8 bit constant within the instruction
    Immediate,

    /// Reference the first 256 bytes of memory
    /// ($0000 to $00FF)
    ZeroPage,

    /// Using [`Self::ZeroPage`] plus the value in the X register
    ZeroPageX,

    /// Using [`Self::ZeroPage`] plus the value in the Y register.
    /// This can only be used by the [`Instruction::LDX`] and [`Instruction::STX`] instructions
    ZeroPageY,

    /// Used in branch instructions which contain a signed 8bit relative offset
    Relative,

    /// Contains a full 16 bit address to identify the target location
    Absolute,

    /// Using [`Self::Absolute`] plus the value in the X register
    AbsoluteX,

    /// Using [`Self::Absolute`] plus the value in the Y register
    AbsoluteY,

    /// [`Instruction::JMP`] is the only supported instruction.
    /// Contains a 16 bit address which identifies the location of the real target
    Indirect,

    /// Indexed indirect addressing is normally used in conjunction 
    /// with a table of address held on zero page. 
    /// The address of the table is taken from the instruction and the X register added to it 
    /// (with zero page wrap around) to give the location of the least significant byte of the 
    /// target address.
    IndirectX,

    /// Indirect indirect addressing is the most common indirection mode used on the 6502. 
    /// The instruction contains the zero page location of the least significant byte of 
    /// 16 bit address. The Y register is dynamically added to this value to generate the 
    /// actual target address for operation
    IndirectY,
}


/// An instruction for the [`CPU`] to run
pub enum Instruction {
    /// # Load accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    LDA,

    /// # Load X register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageY`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteY`]
    LDX,

    /// # Load Y register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`]
    LDY,

    /// # Store accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    STA,

    /// # Store X register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageY`],
    ///
    /// [`AddressingMode::Absolute`]
    STX,

    /// # Store Y register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`]
    STY,

    /// # Transfer accumulator to X
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    TAX,

    /// # Transfer accumulator to Y
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    TAY,

    /// # Transfer X to accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    TXA,

    /// # Transfer Y to accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    TYA,

    /// # Transfer stack pointer to X
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    TSX,

    /// # Transfer X to stack pointer
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    TXS,

    /// # Push accumulator on stack
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    PHA,

    /// # Push processor status on stack
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    PHP,

    /// # Pull accumulator from stack
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    PLA,

    /// # Pull processor status from stack
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    PLP,

    /// # Logical AND on the accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    AND,

    /// # Logical exclusive OR on the accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    EOR,

    /// # Logical inclusive OR on the accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    ORA,

    /// # Bit test
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::Absolute`]
    BIT,

    /// # Add with carry
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    ADC,

    /// # Subtract with carry
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    SBC,

    /// # Compare accumulator
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`],
    ///
    /// [`AddressingMode::AbsoluteY`],
    ///
    /// [`AddressingMode::IndexedIndirect`],
    ///
    /// [`AddressingMode::IndirectIndexed`]
    CMP,

    /// # Compare X register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::Absolute`]
    CPX,

    /// # Compare Y register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Immediate`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::Absolute`]
    CPY,

    /// # Increment a memory location
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`]
    INC,

    /// # Increment the X register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    INX,

    /// # Increment the Y register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    INY,

    /// # Decrement a memory location
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`]
    DEC,

    /// # Decrement the X register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    DEX,

    /// # Decrement the Y register
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    DEY,

    /// # Arithmetic left-shift
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Accumulator`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`]
    ASL,

    /// # Logical right-shift
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Accumulator`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`]
    LSR,

    /// # Rotate left
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Accumulator`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`]
    ROL,

    /// # Rotate right
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Accumulator`],
    ///
    /// [`AddressingMode::ZeroPage`],
    ///
    /// [`AddressingMode::ZeroPageX`],
    ///
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::AbsoluteX`]
    ROR,

    /// # Jump to another location
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Absolute`],
    ///
    /// [`AddressingMode::Indirect`]
    JMP,

    /// # Jump to a subroutine
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Absolute`]
    JSR,

    /// # Return from subroutine
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    RTS,

    /// # Branch if carry flag is clear
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BCC,

    /// # Branch if carry flag is set
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BCS,

    /// # Branch if zero flag is set
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BEQ,

    /// # Branch if negative flag is set
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BMI,

    /// # Branch if zero flag clear
    /// 
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BNE,

    /// # Branch if zero flag is clear
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BPL,

    /// # Branch if overflow flag is clear
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BVC,

    /// # Branch if overflow flag is set
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Relative`]
    BVS,

    /// # Clear carry flag
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    CLC,

    /// # Clear decimal mode flag
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    CLD,

    /// # Clear interrupt disable flag
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    CLI,

    /// # Clear overflow flag
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    CLV,

    /// # Set carry flag
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    SEC,

    /// # Set decimal mode flag
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    SED,

    /// # Set interrupt disable flag
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    SEI,

    /// # Force an interrupt
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    BRK,

    /// # No operation
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    NOP,

    /// # Return from interrupt
    ///
    /// ### Valid addressing modes:
    /// [`AddressingMode::Implied`]
    RTI,
}


/// Which state the CPU is in for the current instruction
pub enum ExecutionState {
    Fetch,
    Read,
    Execute,
    Write
}