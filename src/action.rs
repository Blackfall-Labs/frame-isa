//! Action codes for the SAM ISA
//!
//! Actions specify what operation to perform. Each action is a 2-byte code
//! organized into categories by the high byte.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Action code (2 bytes)
///
/// Specifies what operation to perform. Actions are organized into categories:
///
/// - `0x00xx` - System actions (NOP, HALT, ERROR, STATUS)
/// - `0x01xx` - Response actions (GREET, CONFIRM, DENY, EXPLAIN, etc.)
/// - `0x02xx` - Query actions (ASK, REQUEST, SEARCH, RETRIEVE)
/// - `0x03xx` - Knowledge actions (DEFINE, DESCRIBE, COMPARE, SUMMARIZE)
/// - `0x04xx` - Skill actions (CALCULATE, SET_TIMER, KNOWLEDGE_SEARCH)
/// - `0x05xx` - Emotion actions (EMPATHY, CONCERN, ENCOURAGEMENT, REASSURE)
/// - `0x06xx` - Template actions (TEMPLATE_LOAD, TEMPLATE_FILL)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Action(pub u16);

impl Action {
    // ========== System Actions (0x0000-0x00FF) ==========
    /// No operation
    pub const NOP: Self = Self(0x0000);
    /// Halt execution
    pub const HALT: Self = Self(0x0001);
    /// Error condition
    pub const ERROR: Self = Self(0x0002);
    /// Status report
    pub const STATUS: Self = Self(0x0003);

    // ========== Response Actions (0x0100-0x01FF) ==========
    /// Greeting response
    pub const GREET: Self = Self(0x0100);
    /// Confirmation response
    pub const CONFIRM: Self = Self(0x0101);
    /// Denial response
    pub const DENY: Self = Self(0x0102);
    /// Explanation response
    pub const EXPLAIN: Self = Self(0x0103);
    /// Clarification response
    pub const CLARIFY: Self = Self(0x0104);
    /// Apology response
    pub const APOLOGIZE: Self = Self(0x0105);
    /// Thank you response
    pub const THANK: Self = Self(0x0106);
    /// Generic respond action
    pub const RESPOND: Self = Self(0x0107);

    // ========== Query Actions (0x0200-0x02FF) ==========
    /// Ask a question
    pub const ASK: Self = Self(0x0200);
    /// Request information
    pub const REQUEST: Self = Self(0x0201);
    /// Search for information
    pub const SEARCH: Self = Self(0x0202);
    /// Retrieve stored information
    pub const RETRIEVE: Self = Self(0x0203);

    // ========== Knowledge Actions (0x0300-0x03FF) ==========
    /// Define a term
    pub const DEFINE: Self = Self(0x0300);
    /// Describe something
    pub const DESCRIBE: Self = Self(0x0301);
    /// Compare items
    pub const COMPARE: Self = Self(0x0302);
    /// Summarize content
    pub const SUMMARIZE: Self = Self(0x0303);
    /// Explain how something works
    pub const EXPLAIN_HOW: Self = Self(0x0304);
    /// Explain why something is
    pub const EXPLAIN_WHY: Self = Self(0x0305);

    // ========== Skill Actions (0x0400-0x04FF) ==========
    /// Perform calculation
    pub const CALCULATE: Self = Self(0x0400);
    /// Set a timer
    pub const SET_TIMER: Self = Self(0x0401);
    /// Search knowledge base
    pub const KNOWLEDGE_SEARCH: Self = Self(0x0402);

    // ========== Emotion Actions (0x0500-0x05FF) ==========
    /// Express empathy
    pub const EMPATHY: Self = Self(0x0500);
    /// Express concern
    pub const CONCERN: Self = Self(0x0501);
    /// Give encouragement
    pub const ENCOURAGEMENT: Self = Self(0x0502);
    /// Reassure the user
    pub const REASSURE: Self = Self(0x0503);

    // ========== Template Actions (0x0600-0x06FF) ==========
    /// Load a template
    pub const TEMPLATE_LOAD: Self = Self(0x0600);
    /// Fill a template
    pub const TEMPLATE_FILL: Self = Self(0x0601);

    // ========== Chain Actions (0x0700-0x07FF) ==========
    /// Chain to another TRM
    pub const CHAIN: Self = Self(0x0700);
    /// Fork to multiple TRMs
    pub const FORK: Self = Self(0x0701);
    /// Merge results from TRMs
    pub const MERGE: Self = Self(0x0702);

    /// Create from raw u16 value
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        Self(value)
    }

    /// Get raw u16 value
    #[inline]
    pub const fn as_u16(&self) -> u16 {
        self.0
    }

    /// Get the category byte (high byte)
    #[inline]
    pub const fn category(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    /// Get the subcategory byte (low byte)
    #[inline]
    pub const fn subcategory(&self) -> u8 {
        self.0 as u8
    }

    /// Check if this is a system action (0x00xx)
    #[inline]
    pub const fn is_system(&self) -> bool {
        self.0 <= 0x00FF
    }

    /// Check if this is a response action (0x01xx)
    #[inline]
    pub const fn is_response(&self) -> bool {
        self.0 >= 0x0100 && self.0 <= 0x01FF
    }

    /// Check if this is a query action (0x02xx)
    #[inline]
    pub const fn is_query(&self) -> bool {
        self.0 >= 0x0200 && self.0 <= 0x02FF
    }

    /// Check if this is a knowledge action (0x03xx)
    #[inline]
    pub const fn is_knowledge(&self) -> bool {
        self.0 >= 0x0300 && self.0 <= 0x03FF
    }

    /// Check if this is a skill action (0x04xx)
    #[inline]
    pub const fn is_skill(&self) -> bool {
        self.0 >= 0x0400 && self.0 <= 0x04FF
    }

    /// Check if this is an emotion action (0x05xx)
    #[inline]
    pub const fn is_emotion(&self) -> bool {
        self.0 >= 0x0500 && self.0 <= 0x05FF
    }

    /// Check if this is a template action (0x06xx)
    #[inline]
    pub const fn is_template(&self) -> bool {
        self.0 >= 0x0600 && self.0 <= 0x06FF
    }

    /// Check if this is a chain action (0x07xx)
    #[inline]
    pub const fn is_chain(&self) -> bool {
        self.0 >= 0x0700 && self.0 <= 0x07FF
    }

    /// Get the human-readable name for this action
    pub fn name(&self) -> &'static str {
        match *self {
            Self::NOP => "NOP",
            Self::HALT => "HALT",
            Self::ERROR => "ERROR",
            Self::STATUS => "STATUS",
            Self::GREET => "GREET",
            Self::CONFIRM => "CONFIRM",
            Self::DENY => "DENY",
            Self::EXPLAIN => "EXPLAIN",
            Self::CLARIFY => "CLARIFY",
            Self::APOLOGIZE => "APOLOGIZE",
            Self::THANK => "THANK",
            Self::RESPOND => "RESPOND",
            Self::ASK => "ASK",
            Self::REQUEST => "REQUEST",
            Self::SEARCH => "SEARCH",
            Self::RETRIEVE => "RETRIEVE",
            Self::DEFINE => "DEFINE",
            Self::DESCRIBE => "DESCRIBE",
            Self::COMPARE => "COMPARE",
            Self::SUMMARIZE => "SUMMARIZE",
            Self::EXPLAIN_HOW => "EXPLAIN_HOW",
            Self::EXPLAIN_WHY => "EXPLAIN_WHY",
            Self::CALCULATE => "CALCULATE",
            Self::SET_TIMER => "SET_TIMER",
            Self::KNOWLEDGE_SEARCH => "KNOWLEDGE_SEARCH",
            Self::EMPATHY => "EMPATHY",
            Self::CONCERN => "CONCERN",
            Self::ENCOURAGEMENT => "ENCOURAGEMENT",
            Self::REASSURE => "REASSURE",
            Self::TEMPLATE_LOAD => "TEMPLATE_LOAD",
            Self::TEMPLATE_FILL => "TEMPLATE_FILL",
            Self::CHAIN => "CHAIN",
            Self::FORK => "FORK",
            Self::MERGE => "MERGE",
            _ => "UNKNOWN",
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ACT(0x{:04X}:{})", self.0, self.name())
    }
}

impl From<u16> for Action {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Action> for u16 {
    fn from(action: Action) -> Self {
        action.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_categories() {
        assert!(Action::NOP.is_system());
        assert!(Action::GREET.is_response());
        assert!(Action::SEARCH.is_query());
        assert!(Action::DEFINE.is_knowledge());
        assert!(Action::CALCULATE.is_skill());
        assert!(Action::EMPATHY.is_emotion());
        assert!(Action::TEMPLATE_LOAD.is_template());
        assert!(Action::CHAIN.is_chain());
    }

    #[test]
    fn test_action_bytes() {
        assert_eq!(Action::GREET.category(), 0x01);
        assert_eq!(Action::GREET.subcategory(), 0x00);
        assert_eq!(Action::CALCULATE.category(), 0x04);
        assert_eq!(Action::CALCULATE.subcategory(), 0x00);
    }

    #[test]
    fn test_action_names() {
        assert_eq!(Action::GREET.name(), "GREET");
        assert_eq!(Action::CALCULATE.name(), "CALCULATE");
        assert_eq!(Action::from_u16(0xFFFF).name(), "UNKNOWN");
    }

    #[test]
    fn test_serialization() {
        let action = Action::GREET;
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: Action = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }
}
