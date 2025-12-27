//! Subject codes for the SAM ISA
//!
//! Subjects identify the topic or entity being discussed. Each subject is a 2-byte
//! code organized into categories by the high byte.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Subject/Topic ID (2 bytes)
///
/// References the topic or entity being discussed. Subjects are organized into categories:
///
/// - `0x00xx` - System subjects (NULL, SELF, USER, CONTEXT)
/// - `0x01xx` - Common topics (WEATHER, TIME, DATE, SCHEDULE, HEALTH, HELP)
/// - `0x02xx` - Math/Science (NUMBER, EQUATION, PHYSICS, CHEMISTRY)
/// - `0x03xx` - Technology (COMPUTER, SOFTWARE, HARDWARE, AI, API)
/// - `0x04xx` - Knowledge (DOCUMENTATION, CONCEPT)
/// - `0x05xx` - Emotions (FEELINGS, STRESS, ANXIETY)
/// - `0xE0xx` - RAG references (dynamic document lookups)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Subject(pub u16);

impl Subject {
    // ========== System Subjects (0x0000-0x00FF) ==========
    /// Null/empty subject
    pub const NULL: Self = Self(0x0000);
    /// The AI system itself
    pub const SELF: Self = Self(0x0001);
    /// The user
    pub const USER: Self = Self(0x0002);
    /// Current context
    pub const CONTEXT: Self = Self(0x0003);

    // ========== Common Topics (0x0100-0x01FF) ==========
    /// Weather information
    pub const WEATHER: Self = Self(0x0100);
    /// Time of day
    pub const TIME: Self = Self(0x0101);
    /// Calendar date
    pub const DATE: Self = Self(0x0102);
    /// Schedule/hours
    pub const SCHEDULE: Self = Self(0x0103);
    /// Health topics
    pub const HEALTH: Self = Self(0x0104);
    /// Help/assistance
    pub const HELP: Self = Self(0x0105);
    /// Timezone
    pub const TIMEZONE: Self = Self(0x0106);

    // ========== Math/Science (0x0200-0x02FF) ==========
    /// Numeric value
    pub const NUMBER: Self = Self(0x0200);
    /// Mathematical equation
    pub const EQUATION: Self = Self(0x0201);
    /// Physics topic
    pub const PHYSICS: Self = Self(0x0202);
    /// Chemistry topic
    pub const CHEMISTRY: Self = Self(0x0203);

    // ========== Technology (0x0300-0x03FF) ==========
    /// Computer systems
    pub const COMPUTER: Self = Self(0x0300);
    /// Software
    pub const SOFTWARE: Self = Self(0x0301);
    /// Hardware
    pub const HARDWARE: Self = Self(0x0302);
    /// Artificial intelligence
    pub const AI: Self = Self(0x0303);
    /// API/interface
    pub const API: Self = Self(0x0304);

    // ========== Knowledge (0x0400-0x04FF) ==========
    /// Documentation
    pub const DOCUMENTATION: Self = Self(0x0400);
    /// Abstract concept
    pub const CONCEPT: Self = Self(0x0401);

    // ========== Emotions (0x0500-0x05FF) ==========
    /// General feelings
    pub const FEELINGS: Self = Self(0x0500);
    /// Stress
    pub const STRESS: Self = Self(0x0501);
    /// Anxiety
    pub const ANXIETY: Self = Self(0x0502);

    // ========== TRM References (0x0600-0x06FF) ==========
    /// Reference to another TRM model
    pub const TRM_REF_START: u16 = 0x0600;
    pub const TRM_REF_END: u16 = 0x06FF;

    // ========== RAG References (0xE000-0xEFFF) ==========
    /// Start of RAG reference range
    pub const RAG_START: u16 = 0xE000;
    /// End of RAG reference range
    pub const RAG_END: u16 = 0xEFFF;

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

    /// Check if this is a RAG reference (requires document lookup)
    #[inline]
    pub const fn is_rag_reference(&self) -> bool {
        self.0 >= Self::RAG_START && self.0 <= Self::RAG_END
    }

    /// Check if this is a TRM reference (chains to another model)
    #[inline]
    pub const fn is_trm_reference(&self) -> bool {
        self.0 >= Self::TRM_REF_START && self.0 <= Self::TRM_REF_END
    }

    /// Check if this is a system subject (0x00xx)
    #[inline]
    pub const fn is_system(&self) -> bool {
        self.0 <= 0x00FF
    }

    /// Check if this is a common topic (0x01xx)
    #[inline]
    pub const fn is_common_topic(&self) -> bool {
        self.0 >= 0x0100 && self.0 <= 0x01FF
    }

    /// Check if this is a math/science subject (0x02xx)
    #[inline]
    pub const fn is_math_science(&self) -> bool {
        self.0 >= 0x0200 && self.0 <= 0x02FF
    }

    /// Check if this is a technology subject (0x03xx)
    #[inline]
    pub const fn is_technology(&self) -> bool {
        self.0 >= 0x0300 && self.0 <= 0x03FF
    }

    /// Check if this is a knowledge subject (0x04xx)
    #[inline]
    pub const fn is_knowledge(&self) -> bool {
        self.0 >= 0x0400 && self.0 <= 0x04FF
    }

    /// Check if this is an emotion subject (0x05xx)
    #[inline]
    pub const fn is_emotion(&self) -> bool {
        self.0 >= 0x0500 && self.0 <= 0x05FF
    }

    /// Get the human-readable name for this subject
    pub fn name(&self) -> &'static str {
        match *self {
            Self::NULL => "NULL",
            Self::SELF => "SELF",
            Self::USER => "USER",
            Self::CONTEXT => "CONTEXT",
            Self::WEATHER => "WEATHER",
            Self::TIME => "TIME",
            Self::DATE => "DATE",
            Self::SCHEDULE => "SCHEDULE",
            Self::HEALTH => "HEALTH",
            Self::HELP => "HELP",
            Self::TIMEZONE => "TIMEZONE",
            Self::NUMBER => "NUMBER",
            Self::EQUATION => "EQUATION",
            Self::PHYSICS => "PHYSICS",
            Self::CHEMISTRY => "CHEMISTRY",
            Self::COMPUTER => "COMPUTER",
            Self::SOFTWARE => "SOFTWARE",
            Self::HARDWARE => "HARDWARE",
            Self::AI => "AI",
            Self::API => "API",
            Self::DOCUMENTATION => "DOCUMENTATION",
            Self::CONCEPT => "CONCEPT",
            Self::FEELINGS => "FEELINGS",
            Self::STRESS => "STRESS",
            Self::ANXIETY => "ANXIETY",
            _ if self.is_rag_reference() => "RAG_REF",
            _ if self.is_trm_reference() => "TRM_REF",
            _ => "UNKNOWN",
        }
    }

    /// Create a RAG reference for a given document ID
    #[inline]
    pub const fn rag_ref(doc_id: u16) -> Self {
        // Clamp to RAG range
        let id = if doc_id > 0x0FFF { 0x0FFF } else { doc_id };
        Self(Self::RAG_START + id)
    }

    /// Create a TRM reference for a given model ID
    #[inline]
    pub const fn trm_ref(model_id: u8) -> Self {
        Self(Self::TRM_REF_START + model_id as u16)
    }

    /// Get the RAG document ID if this is a RAG reference
    #[inline]
    pub const fn rag_doc_id(&self) -> Option<u16> {
        if self.is_rag_reference() {
            Some(self.0 - Self::RAG_START)
        } else {
            None
        }
    }

    /// Get the TRM model ID if this is a TRM reference
    #[inline]
    pub const fn trm_model_id(&self) -> Option<u8> {
        if self.is_trm_reference() {
            Some((self.0 - Self::TRM_REF_START) as u8)
        } else {
            None
        }
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_rag_reference() {
            write!(f, "SUBJ(RAG:0x{:04X})", self.0)
        } else if self.is_trm_reference() {
            write!(f, "SUBJ(TRM:0x{:02X})", self.trm_model_id().unwrap())
        } else {
            write!(f, "SUBJ(0x{:04X}:{})", self.0, self.name())
        }
    }
}

impl From<u16> for Subject {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Subject> for u16 {
    fn from(subject: Subject) -> Self {
        subject.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_categories() {
        assert!(Subject::NULL.is_system());
        assert!(Subject::WEATHER.is_common_topic());
        assert!(Subject::TIME.is_common_topic());
        assert!(Subject::NUMBER.is_math_science());
        assert!(Subject::API.is_technology());
        assert!(Subject::DOCUMENTATION.is_knowledge());
        assert!(Subject::STRESS.is_emotion());
    }

    #[test]
    fn test_rag_reference() {
        let rag_ref = Subject::rag_ref(0x0A3);
        assert!(rag_ref.is_rag_reference());
        assert_eq!(rag_ref.rag_doc_id(), Some(0x0A3));
        assert_eq!(rag_ref.as_u16(), 0xE0A3);

        assert!(!Subject::USER.is_rag_reference());
        assert_eq!(Subject::USER.rag_doc_id(), None);
    }

    #[test]
    fn test_trm_reference() {
        let trm_ref = Subject::trm_ref(5);
        assert!(trm_ref.is_trm_reference());
        assert_eq!(trm_ref.trm_model_id(), Some(5));
        assert_eq!(trm_ref.as_u16(), 0x0605);

        assert!(!Subject::USER.is_trm_reference());
    }

    #[test]
    fn test_serialization() {
        let subject = Subject::TIME;
        let json = serde_json::to_string(&subject).unwrap();
        let deserialized: Subject = serde_json::from_str(&json).unwrap();
        assert_eq!(subject, deserialized);
    }
}
