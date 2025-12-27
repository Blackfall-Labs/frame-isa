//! Instruction representation for the SAM ISA
//!
//! A complete 6-byte instruction consisting of Action, Subject, and Modifier.

use crate::{Action, Modifier, Subject};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Size of a single instruction in bytes
pub const INSTRUCTION_SIZE: usize = 6;

/// A single 6-byte instruction (ACT + SUBJ + MOD)
///
/// Format (big-endian):
/// ```text
/// Byte:  0     1     2     3     4     5
///       [ACT_HI][ACT_LO][SUBJ_HI][SUBJ_LO][MOD_HI][MOD_LO]
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Instruction {
    /// The action to perform
    pub action: Action,
    /// The subject/topic
    pub subject: Subject,
    /// Style modifiers
    pub modifier: Modifier,
}

impl Instruction {
    /// Create a new instruction
    #[inline]
    pub const fn new(action: Action, subject: Subject, modifier: Modifier) -> Self {
        Self {
            action,
            subject,
            modifier,
        }
    }

    /// Create an instruction with default modifier
    #[inline]
    pub fn simple(action: Action, subject: Subject) -> Self {
        Self::new(action, subject, Modifier::default())
    }

    /// Parse instructions from byte array
    ///
    /// Expects bytes in format: [ACT_HIGH, ACT_LOW, SUBJ_HIGH, SUBJ_LOW, MOD_HIGH, MOD_LOW, ...]
    pub fn parse_all(bytes: &[u8]) -> Result<Vec<Self>, InstructionError> {
        if bytes.len() % INSTRUCTION_SIZE != 0 {
            return Err(InstructionError::InvalidLength {
                actual: bytes.len(),
                expected_multiple_of: INSTRUCTION_SIZE,
            });
        }

        let mut instructions = Vec::with_capacity(bytes.len() / INSTRUCTION_SIZE);
        for chunk in bytes.chunks_exact(INSTRUCTION_SIZE) {
            instructions.push(Self::parse_one(chunk)?);
        }

        Ok(instructions)
    }

    /// Parse a single instruction from exactly 6 bytes
    pub fn parse_one(bytes: &[u8]) -> Result<Self, InstructionError> {
        if bytes.len() != INSTRUCTION_SIZE {
            return Err(InstructionError::InvalidLength {
                actual: bytes.len(),
                expected_multiple_of: INSTRUCTION_SIZE,
            });
        }

        let action = Action::from_u16(u16::from_be_bytes([bytes[0], bytes[1]]));
        let subject = Subject::from_u16(u16::from_be_bytes([bytes[2], bytes[3]]));
        let modifier = Modifier::from_u16(u16::from_be_bytes([bytes[4], bytes[5]]));

        Ok(Self::new(action, subject, modifier))
    }

    /// Serialize instruction to 6 bytes (big-endian)
    pub fn to_bytes(&self) -> [u8; INSTRUCTION_SIZE] {
        let action_bytes = self.action.as_u16().to_be_bytes();
        let subject_bytes = self.subject.as_u16().to_be_bytes();
        let modifier_bytes = self.modifier.as_u16().to_be_bytes();

        [
            action_bytes[0],
            action_bytes[1],
            subject_bytes[0],
            subject_bytes[1],
            modifier_bytes[0],
            modifier_bytes[1],
        ]
    }

    /// Serialize multiple instructions to bytes
    pub fn to_bytes_all(instructions: &[Self]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(instructions.len() * INSTRUCTION_SIZE);
        for instr in instructions {
            bytes.extend_from_slice(&instr.to_bytes());
        }
        bytes
    }

    /// Check if this instruction requires RAG lookup
    #[inline]
    pub const fn needs_rag(&self) -> bool {
        self.subject.is_rag_reference()
    }

    /// Check if this instruction chains to another TRM
    #[inline]
    pub const fn is_chain(&self) -> bool {
        self.action.is_chain() || self.subject.is_trm_reference()
    }

    /// Check if this is a system instruction
    #[inline]
    pub const fn is_system(&self) -> bool {
        self.action.is_system()
    }

    /// Get a compact string representation
    pub fn to_opcode_string(&self) -> String {
        format!(
            "{:04X}:{:04X}:{:04X}",
            self.action.as_u16(),
            self.subject.as_u16(),
            self.modifier.as_u16()
        )
    }

    /// Parse from compact opcode string (e.g., "0100:0101:0050")
    pub fn from_opcode_string(s: &str) -> Result<Self, InstructionError> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(InstructionError::InvalidOpcodeString(s.to_string()));
        }

        let action = u16::from_str_radix(parts[0], 16)
            .map_err(|_| InstructionError::InvalidOpcodeString(s.to_string()))?;
        let subject = u16::from_str_radix(parts[1], 16)
            .map_err(|_| InstructionError::InvalidOpcodeString(s.to_string()))?;
        let modifier = u16::from_str_radix(parts[2], 16)
            .map_err(|_| InstructionError::InvalidOpcodeString(s.to_string()))?;

        Ok(Self::new(
            Action::from_u16(action),
            Subject::from_u16(subject),
            Modifier::from_u16(modifier),
        ))
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} | {} | {}]", self.action, self.subject, self.modifier)
    }
}

/// Errors that can occur when parsing instructions
#[derive(Debug, Error)]
pub enum InstructionError {
    #[error("Invalid byte length: got {actual}, expected multiple of {expected_multiple_of}")]
    InvalidLength {
        actual: usize,
        expected_multiple_of: usize,
    },

    #[error("Invalid opcode string: {0}")]
    InvalidOpcodeString(String),
}

/// Builder for constructing instructions fluently
#[derive(Debug, Clone)]
pub struct InstructionBuilder {
    action: Action,
    subject: Subject,
    modifier: Modifier,
}

impl InstructionBuilder {
    /// Start building with an action
    pub fn new(action: Action) -> Self {
        Self {
            action,
            subject: Subject::NULL,
            modifier: Modifier::default(),
        }
    }

    /// Set the subject
    pub fn subject(mut self, subject: Subject) -> Self {
        self.subject = subject;
        self
    }

    /// Set the modifier
    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Set voice style
    pub fn voice(mut self, voice: crate::modifier::Voice) -> Self {
        self.modifier = self.modifier.with_voice(voice);
        self
    }

    /// Set tone
    pub fn tone(mut self, tone: crate::modifier::Tone) -> Self {
        self.modifier = self.modifier.with_tone(tone);
        self
    }

    /// Set warmth
    pub fn warmth(mut self, warmth: crate::modifier::Warmth) -> Self {
        self.modifier = self.modifier.with_warmth(warmth);
        self
    }

    /// Set format
    pub fn format(mut self, format: crate::modifier::Format) -> Self {
        self.modifier = self.modifier.with_format(format);
        self
    }

    /// Set urgency
    pub fn urgency(mut self, urgency: crate::modifier::Urgency) -> Self {
        self.modifier = self.modifier.with_urgency(urgency);
        self
    }

    /// Build the instruction
    pub fn build(self) -> Instruction {
        Instruction::new(self.action, self.subject, self.modifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_instruction() {
        // GREET USER with some modifier
        let bytes = vec![0x01, 0x00, 0x00, 0x02, 0x08, 0x10];
        let instr = Instruction::parse_one(&bytes).unwrap();

        assert_eq!(instr.action, Action::GREET);
        assert_eq!(instr.subject, Subject::USER);
        assert_eq!(instr.modifier.as_u16(), 0x0810);
    }

    #[test]
    fn test_parse_multiple_instructions() {
        let bytes = vec![
            0x01, 0x00, 0x00, 0x02, 0x08, 0x10, // GREET USER
            0x03, 0x00, 0x03, 0x04, 0xC3, 0x00, // DEFINE API
        ];

        let instructions = Instruction::parse_all(&bytes).unwrap();
        assert_eq!(instructions.len(), 2);

        assert_eq!(instructions[0].action, Action::GREET);
        assert_eq!(instructions[1].action, Action::DEFINE);
        assert_eq!(instructions[1].subject, Subject::API);
    }

    #[test]
    fn test_to_bytes_roundtrip() {
        let original = Instruction::new(
            Action::CALCULATE,
            Subject::NUMBER,
            Modifier::VOICE_TECHNICAL,
        );

        let bytes = original.to_bytes();
        let parsed = Instruction::parse_one(&bytes).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_opcode_string_roundtrip() {
        let instr = Instruction::new(Action::RESPOND, Subject::TIME, Modifier::default());

        let opcode_str = instr.to_opcode_string();
        let parsed = Instruction::from_opcode_string(&opcode_str).unwrap();

        assert_eq!(instr, parsed);
    }

    #[test]
    fn test_invalid_length() {
        let bytes = vec![0x01, 0x00, 0x00]; // Only 3 bytes
        let result = Instruction::parse_one(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_needs_rag() {
        let rag_instr = Instruction::new(
            Action::DESCRIBE,
            Subject::rag_ref(0x0A3),
            Modifier::default(),
        );
        assert!(rag_instr.needs_rag());

        let normal_instr = Instruction::new(Action::GREET, Subject::USER, Modifier::default());
        assert!(!normal_instr.needs_rag());
    }

    #[test]
    fn test_is_chain() {
        let chain_instr = Instruction::new(Action::CHAIN, Subject::trm_ref(5), Modifier::default());
        assert!(chain_instr.is_chain());

        let normal_instr = Instruction::new(Action::GREET, Subject::USER, Modifier::default());
        assert!(!normal_instr.is_chain());
    }

    #[test]
    fn test_builder() {
        use crate::modifier::{Tone, Voice, Warmth};

        let instr = InstructionBuilder::new(Action::RESPOND)
            .subject(Subject::TIME)
            .voice(Voice::Casual)
            .tone(Tone::Positive)
            .warmth(Warmth::Warm)
            .build();

        assert_eq!(instr.action, Action::RESPOND);
        assert_eq!(instr.subject, Subject::TIME);
        assert_eq!(instr.modifier.voice(), Voice::Casual);
        assert_eq!(instr.modifier.tone(), Tone::Positive);
    }

    #[test]
    fn test_simple_constructor() {
        let instr = Instruction::simple(Action::GREET, Subject::USER);
        assert_eq!(instr.modifier, Modifier::default());
    }
}
