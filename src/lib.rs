//! # Frame ISA - SAM Instruction Set Architecture
//!
//! Zero-dependency crate defining the 6-byte opcode format for SAM AI systems.
//!
//! ## Overview
//!
//! Frame ISA defines a compact instruction format for AI output:
//!
//! ```text
//! [ACT:2 bytes][SUBJ:2 bytes][MOD:2 bytes] = 6 bytes total (big-endian)
//! ```
//!
//! - **ACT (Action)**: What operation to perform (GREET, RESPOND, CALCULATE, etc.)
//! - **SUBJ (Subject)**: The topic or entity (TIME, USER, WEATHER, RAG reference, etc.)
//! - **MOD (Modifier)**: Style flags (voice, tone, warmth, format, urgency, etc.)
//!
//! ## Usage
//!
//! ```rust
//! use frame_isa::{Action, Subject, Modifier, Instruction};
//!
//! // Create an instruction
//! let instr = Instruction::new(
//!     Action::RESPOND,
//!     Subject::TIME,
//!     Modifier::default(),
//! );
//!
//! // Serialize to bytes
//! let bytes = instr.to_bytes();
//! assert_eq!(bytes.len(), 6);
//!
//! // Parse from bytes
//! let parsed = Instruction::parse_one(&bytes).unwrap();
//! assert_eq!(instr, parsed);
//!
//! // Use the builder
//! use frame_isa::{InstructionBuilder, Voice, Tone};
//!
//! let instr = InstructionBuilder::new(Action::GREET)
//!     .subject(Subject::USER)
//!     .voice(Voice::Casual)
//!     .tone(Tone::Positive)
//!     .build();
//! ```
//!
//! ## Action Categories
//!
//! | Range | Category | Examples |
//! |-------|----------|----------|
//! | 0x00xx | System | NOP, HALT, ERROR, STATUS |
//! | 0x01xx | Response | GREET, CONFIRM, DENY, EXPLAIN, RESPOND |
//! | 0x02xx | Query | ASK, REQUEST, SEARCH, RETRIEVE |
//! | 0x03xx | Knowledge | DEFINE, DESCRIBE, COMPARE, SUMMARIZE |
//! | 0x04xx | Skill | CALCULATE, SET_TIMER, KNOWLEDGE_SEARCH |
//! | 0x05xx | Emotion | EMPATHY, CONCERN, ENCOURAGEMENT |
//! | 0x06xx | Template | TEMPLATE_LOAD, TEMPLATE_FILL |
//! | 0x07xx | Chain | CHAIN, FORK, MERGE |
//!
//! ## Subject Categories
//!
//! | Range | Category | Examples |
//! |-------|----------|----------|
//! | 0x00xx | System | NULL, SELF, USER, CONTEXT |
//! | 0x01xx | Common | WEATHER, TIME, DATE, SCHEDULE |
//! | 0x02xx | Math/Science | NUMBER, EQUATION, PHYSICS |
//! | 0x03xx | Technology | COMPUTER, SOFTWARE, AI, API |
//! | 0x04xx | Knowledge | DOCUMENTATION, CONCEPT |
//! | 0x05xx | Emotions | FEELINGS, STRESS, ANXIETY |
//! | 0x06xx | TRM Refs | References to other TRM models |
//! | 0xE0xx | RAG Refs | Dynamic document lookups |
//!
//! ## Modifier Bit Layout
//!
//! ```text
//! Bit:  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0
//!       [--VOICE--] [--TONE--] [-WARM-] [--FORMAT--] [ACCURACY] [URGENCY]
//! ```
//!
//! ## Integration with TRM Models
//!
//! TinyRecursiveModels (TRMs) output predictions that map to these opcodes.
//! The factored prediction approach uses 3 heads:
//!
//! - ACT head → Action code
//! - SUBJ head → Subject code
//! - MOD head → Modifier flags
//!
//! This allows small models (~148K params) to achieve 99%+ accuracy on
//! opcode prediction tasks.

pub mod action;
pub mod instruction;
pub mod modifier;
pub mod subject;

// Re-export main types
pub use action::Action;
pub use instruction::{Instruction, InstructionBuilder, InstructionError, INSTRUCTION_SIZE};
pub use modifier::{Accuracy, Format, Modifier, Tone, Urgency, Voice, Warmth};
pub use subject::Subject;

/// Current ISA version
pub const ISA_VERSION: &str = "0.1.0";

/// Convenience prelude for common imports
pub mod prelude {
    pub use crate::action::Action;
    pub use crate::instruction::{Instruction, InstructionBuilder, INSTRUCTION_SIZE};
    pub use crate::modifier::{Accuracy, Format, Modifier, Tone, Urgency, Voice, Warmth};
    pub use crate::subject::Subject;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_to_end() {
        // Create instruction using builder
        let instr = InstructionBuilder::new(Action::RESPOND)
            .subject(Subject::TIME)
            .voice(Voice::Casual)
            .tone(Tone::Positive)
            .build();

        // Serialize
        let bytes = instr.to_bytes();
        assert_eq!(bytes.len(), INSTRUCTION_SIZE);

        // Parse back
        let parsed = Instruction::parse_one(&bytes).unwrap();
        assert_eq!(instr, parsed);

        // Verify fields
        assert_eq!(parsed.action, Action::RESPOND);
        assert_eq!(parsed.subject, Subject::TIME);
        assert_eq!(parsed.modifier.voice(), Voice::Casual);
        assert_eq!(parsed.modifier.tone(), Tone::Positive);
    }

    #[test]
    fn test_rag_chain() {
        // Create a chain instruction that references another TRM
        let chain = Instruction::new(Action::CHAIN, Subject::trm_ref(3), Modifier::default());

        assert!(chain.is_chain());
        assert_eq!(chain.subject.trm_model_id(), Some(3));

        // Create a RAG instruction
        let rag = Instruction::new(
            Action::RETRIEVE,
            Subject::rag_ref(0x42),
            Modifier::default(),
        );

        assert!(rag.needs_rag());
        assert_eq!(rag.subject.rag_doc_id(), Some(0x42));
    }

    #[test]
    fn test_opcode_string() {
        let instr = Instruction::new(Action::GREET, Subject::USER, Modifier::default());

        let opcode_str = instr.to_opcode_string();
        assert!(opcode_str.contains(":"));

        let parsed = Instruction::from_opcode_string(&opcode_str).unwrap();
        assert_eq!(instr, parsed);
    }
}
