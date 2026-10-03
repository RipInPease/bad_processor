mod run_instruction;

struct CPU {
    registers: Registers,
    ram: RAM,
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


/// Where in memory to read or write for a certain [`Instruction`]
pub enum AddressingMode {
    /// If the addressing mode is implied for the instruction
    Implied,

    /// Work directly on the accumulator
    Accumulator,

    /// Immediate addressing allows the programmer to directly specify an 
    /// 8 bit constant within the instruction
    Immediate(u8),

    /// Reference the first 256 bytes of memory
    /// ($0000 to $00FF)
    ZeroPage(u8),

    /// Using [`Self::ZeroPage`] plus the value in the X register
    ZeroPageX(u8),

    /// Using [`Self::ZeroPage`] plus the value in the Y register.
    /// This can only be used by the [`Instruction::LDX`] and [`Instruction::STX`] instructions
    ZeroPageY(u8),

    /// Used in branch instructions which contain a signed 8bit relative offset
    Relative(i8),

    /// Contains a full 16 bit address to identify the target location
    Absolute(u16),

    /// Using [`Self::Absolute`] plus the value in the X register
    AbsoluteX(u16),

    /// Using [`Self::Absolute`] plus the value in the Y register
    AbsoluteY(u16),

    /// [`Instruction::JMP`] is the only supported instruction.
    /// Contains a 16 bit address which identifies the location of the real target
    Indirect(u16),

    /// Indexed indirect addressing is normally used in conjunction 
    /// with a table of address held on zero page. 
    /// The address of the table is taken from the instruction and the X register added to it 
    /// (with zero page wrap around) to give the location of the least significant byte of the 
    /// target address.
    IndexedIndirect(u8),

    /// Indirect indirect addressing is the most common indirection mode used on the 6502. 
    /// The instruction contains the zero page location of the least significant byte of 
    /// 16 bit address. The Y register is dynamically added to this value to generate the 
    /// actual target address for operation
    IndirectIndexed(u8),
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

