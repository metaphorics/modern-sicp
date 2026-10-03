// SPDX-License-Identifier: GPL-3.0-only

//! The register-machine data language of sections 5.1 and 5.2
//! (grammar §7): registers, labels, operands, and instructions as
//! typed values. Machine programs are built with these constructors;
//! the source grammar claims no controller text, and an unbound label
//! or register is a typed machine error, never an invented source
//! feature.

/// A machine register name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Register(pub String);

/// A controller label name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

/// One instruction operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// An integer constant.
    Constant(i64),
    /// A register read.
    Register(Register),
    /// A label reference.
    Label(Label),
    /// A named operation applied to operands: computation results flow
    /// through `Assign { target, value: Operation { .. } }`. This is
    /// the recorded contract repair of grammar §7: `Perform` remains
    /// effect-only and never writes a register.
    Operation {
        /// The operation's name.
        operation: String,
        /// The operation's arguments.
        arguments: Vec<Operand>,
    },
}

/// One machine instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// `assign`: write an operand into a register.
    Assign {
        /// The destination register.
        target: Register,
        /// The source operand.
        value: Operand,
    },
    /// `test`: set the machine's flag from a named predicate.
    Test {
        /// The predicate's name.
        predicate: String,
        /// The predicate arguments.
        arguments: Vec<Operand>,
    },
    /// `branch`: continue at a label when the flag is set.
    Branch(Label),
    /// `goto`: continue at a register or label operand.
    Goto(Operand),
    /// `save`: push a register onto the explicit stack.
    Save(Register),
    /// `restore`: pop the explicit stack into a register.
    Restore(Register),
    /// `perform`: run a named operation for its effects.
    Perform {
        /// The operation's name.
        operation: String,
        /// The operation arguments.
        arguments: Vec<Operand>,
    },
}

/// A machine program: its registers and its labeled instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineProgram {
    /// The registers, in declaration order.
    pub registers: Vec<Register>,
    /// The instructions with their optional leading labels.
    pub instructions: Vec<(Option<Label>, Instruction)>,
}

impl MachineProgram {
    /// Builds a machine program from its parts.
    #[must_use]
    pub fn new(registers: Vec<Register>, instructions: Vec<(Option<Label>, Instruction)>) -> Self {
        Self {
            registers,
            instructions,
        }
    }
}
