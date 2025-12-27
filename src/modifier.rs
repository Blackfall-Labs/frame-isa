//! Modifier flags for the SAM ISA
//!
//! Modifiers control the style and attributes of opcode output. Each modifier is
//! a 2-byte bit-packed value with multiple fields.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Modifier flags (2 bytes)
///
/// Controls styling and attributes of opcode output. Bit layout:
///
/// ```text
/// Bit:  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0
///       [--VOICE--] [--TONE--] [-WARM-] [--FORMAT--] [ACCURACY] [URGENCY]
/// ```
///
/// - **Voice** (bits 15-14): Speaking style - Neutral, Formal, Casual, Technical
/// - **Tone** (bits 13-12): Emotional tone - Neutral, Positive, Empathetic, Cautious
/// - **Warmth** (bits 11-10): Interpersonal warmth - Cold, Neutral, Warm, VeryWarm
/// - **Format** (bits 9-8): Output format - Prose, Bulleted, Numbered, Structured
/// - **Accuracy** (bits 7-6): Confidence level - Low, Medium, High, Verified
/// - **Urgency** (bits 5-4): Priority level - Low, Normal, High, Critical
/// - **Reserved** (bits 3-0): For future use
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Modifier(pub u16);

impl Modifier {
    // ========== Voice Styles (bits 15-14) ==========
    pub const VOICE_NEUTRAL: Self = Self(0x0000);
    pub const VOICE_FORMAL: Self = Self(0x4000);
    pub const VOICE_CASUAL: Self = Self(0x8000);
    pub const VOICE_TECHNICAL: Self = Self(0xC000);

    // ========== Tone (bits 13-12) ==========
    pub const TONE_NEUTRAL: Self = Self(0x0000);
    pub const TONE_POSITIVE: Self = Self(0x1000);
    pub const TONE_EMPATHETIC: Self = Self(0x2000);
    pub const TONE_CAUTIOUS: Self = Self(0x3000);

    // ========== Warmth (bits 11-10) ==========
    pub const WARMTH_COLD: Self = Self(0x0000);
    pub const WARMTH_NEUTRAL: Self = Self(0x0400);
    pub const WARMTH_WARM: Self = Self(0x0800);
    pub const WARMTH_VERY_WARM: Self = Self(0x0C00);

    // ========== Format (bits 9-8) ==========
    pub const FORMAT_PROSE: Self = Self(0x0000);
    pub const FORMAT_BULLETED: Self = Self(0x0100);
    pub const FORMAT_NUMBERED: Self = Self(0x0200);
    pub const FORMAT_STRUCTURED: Self = Self(0x0300);

    // ========== Accuracy (bits 7-6) ==========
    pub const ACCURACY_LOW: Self = Self(0x0000);
    pub const ACCURACY_MEDIUM: Self = Self(0x0040);
    pub const ACCURACY_HIGH: Self = Self(0x0080);
    pub const ACCURACY_VERIFIED: Self = Self(0x00C0);

    // ========== Urgency (bits 5-4) ==========
    pub const URGENCY_LOW: Self = Self(0x0000);
    pub const URGENCY_NORMAL: Self = Self(0x0010);
    pub const URGENCY_HIGH: Self = Self(0x0020);
    pub const URGENCY_CRITICAL: Self = Self(0x0030);

    // ========== Bit Masks ==========
    const VOICE_MASK: u16 = 0xC000;
    const TONE_MASK: u16 = 0x3000;
    const WARMTH_MASK: u16 = 0x0C00;
    const FORMAT_MASK: u16 = 0x0300;
    const ACCURACY_MASK: u16 = 0x00C0;
    const URGENCY_MASK: u16 = 0x0030;

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

    /// Get voice style
    #[inline]
    pub const fn voice(&self) -> Voice {
        match self.0 & Self::VOICE_MASK {
            0x0000 => Voice::Neutral,
            0x4000 => Voice::Formal,
            0x8000 => Voice::Casual,
            _ => Voice::Technical,
        }
    }

    /// Get tone
    #[inline]
    pub const fn tone(&self) -> Tone {
        match self.0 & Self::TONE_MASK {
            0x0000 => Tone::Neutral,
            0x1000 => Tone::Positive,
            0x2000 => Tone::Empathetic,
            _ => Tone::Cautious,
        }
    }

    /// Get warmth level
    #[inline]
    pub const fn warmth(&self) -> Warmth {
        match self.0 & Self::WARMTH_MASK {
            0x0000 => Warmth::Cold,
            0x0400 => Warmth::Neutral,
            0x0800 => Warmth::Warm,
            _ => Warmth::VeryWarm,
        }
    }

    /// Get output format
    #[inline]
    pub const fn format(&self) -> Format {
        match self.0 & Self::FORMAT_MASK {
            0x0000 => Format::Prose,
            0x0100 => Format::Bulleted,
            0x0200 => Format::Numbered,
            _ => Format::Structured,
        }
    }

    /// Get accuracy level
    #[inline]
    pub const fn accuracy(&self) -> Accuracy {
        match self.0 & Self::ACCURACY_MASK {
            0x0000 => Accuracy::Low,
            0x0040 => Accuracy::Medium,
            0x0080 => Accuracy::High,
            _ => Accuracy::Verified,
        }
    }

    /// Get urgency level
    #[inline]
    pub const fn urgency(&self) -> Urgency {
        match self.0 & Self::URGENCY_MASK {
            0x0000 => Urgency::Low,
            0x0010 => Urgency::Normal,
            0x0020 => Urgency::High,
            _ => Urgency::Critical,
        }
    }

    /// Set voice style
    #[inline]
    pub const fn with_voice(self, voice: Voice) -> Self {
        let voice_bits = match voice {
            Voice::Neutral => 0x0000,
            Voice::Formal => 0x4000,
            Voice::Casual => 0x8000,
            Voice::Technical => 0xC000,
        };
        Self((self.0 & !Self::VOICE_MASK) | voice_bits)
    }

    /// Set tone
    #[inline]
    pub const fn with_tone(self, tone: Tone) -> Self {
        let tone_bits = match tone {
            Tone::Neutral => 0x0000,
            Tone::Positive => 0x1000,
            Tone::Empathetic => 0x2000,
            Tone::Cautious => 0x3000,
        };
        Self((self.0 & !Self::TONE_MASK) | tone_bits)
    }

    /// Set warmth level
    #[inline]
    pub const fn with_warmth(self, warmth: Warmth) -> Self {
        let warmth_bits = match warmth {
            Warmth::Cold => 0x0000,
            Warmth::Neutral => 0x0400,
            Warmth::Warm => 0x0800,
            Warmth::VeryWarm => 0x0C00,
        };
        Self((self.0 & !Self::WARMTH_MASK) | warmth_bits)
    }

    /// Set output format
    #[inline]
    pub const fn with_format(self, format: Format) -> Self {
        let format_bits = match format {
            Format::Prose => 0x0000,
            Format::Bulleted => 0x0100,
            Format::Numbered => 0x0200,
            Format::Structured => 0x0300,
        };
        Self((self.0 & !Self::FORMAT_MASK) | format_bits)
    }

    /// Set accuracy level
    #[inline]
    pub const fn with_accuracy(self, accuracy: Accuracy) -> Self {
        let accuracy_bits = match accuracy {
            Accuracy::Low => 0x0000,
            Accuracy::Medium => 0x0040,
            Accuracy::High => 0x0080,
            Accuracy::Verified => 0x00C0,
        };
        Self((self.0 & !Self::ACCURACY_MASK) | accuracy_bits)
    }

    /// Set urgency level
    #[inline]
    pub const fn with_urgency(self, urgency: Urgency) -> Self {
        let urgency_bits = match urgency {
            Urgency::Low => 0x0000,
            Urgency::Normal => 0x0010,
            Urgency::High => 0x0020,
            Urgency::Critical => 0x0030,
        };
        Self((self.0 & !Self::URGENCY_MASK) | urgency_bits)
    }

    /// Create a crisis-appropriate modifier (empathetic, warm, high urgency)
    pub const fn crisis() -> Self {
        Self(0x0000)
            .with_tone(Tone::Empathetic)
            .with_warmth(Warmth::VeryWarm)
            .with_urgency(Urgency::High)
            .with_accuracy(Accuracy::High)
    }

    /// Create a professional modifier (formal, neutral warmth)
    pub const fn professional() -> Self {
        Self(0x0000)
            .with_voice(Voice::Formal)
            .with_warmth(Warmth::Neutral)
            .with_accuracy(Accuracy::High)
            .with_urgency(Urgency::Normal)
    }

    /// Create a friendly modifier (casual, warm)
    pub const fn friendly() -> Self {
        Self(0x0000)
            .with_voice(Voice::Casual)
            .with_tone(Tone::Positive)
            .with_warmth(Warmth::Warm)
            .with_urgency(Urgency::Normal)
    }
}

impl Default for Modifier {
    fn default() -> Self {
        // Neutral voice, tone, warmth; prose format; medium accuracy; normal urgency
        Self(0x0400 | 0x0040 | 0x0010) // WARMTH_NEUTRAL | ACCURACY_MEDIUM | URGENCY_NORMAL
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MOD(0x{:04X}: {:?}/{:?}/{:?}/{:?})",
            self.0,
            self.voice(),
            self.tone(),
            self.warmth(),
            self.format()
        )
    }
}

impl From<u16> for Modifier {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Modifier> for u16 {
    fn from(modifier: Modifier) -> Self {
        modifier.0
    }
}

// ========== Field Enums ==========

/// Voice style for output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Voice {
    /// Default neutral voice
    Neutral,
    /// Formal/professional voice
    Formal,
    /// Casual/conversational voice
    Casual,
    /// Technical/precise voice
    Technical,
}

/// Emotional tone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tone {
    /// Neutral tone
    Neutral,
    /// Positive/upbeat tone
    Positive,
    /// Empathetic/understanding tone
    Empathetic,
    /// Cautious/careful tone
    Cautious,
}

/// Interpersonal warmth level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Warmth {
    /// Cold/distant
    Cold,
    /// Neutral warmth
    Neutral,
    /// Warm/friendly
    Warm,
    /// Very warm/caring
    VeryWarm,
}

/// Output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    /// Prose/paragraph format
    Prose,
    /// Bulleted list
    Bulleted,
    /// Numbered list
    Numbered,
    /// Structured/formatted output
    Structured,
}

/// Confidence/accuracy level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Accuracy {
    /// Low confidence
    Low,
    /// Medium confidence
    Medium,
    /// High confidence
    High,
    /// Verified/certain
    Verified,
}

/// Urgency/priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Urgency {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_modifier() {
        let m = Modifier::default();
        assert_eq!(m.voice(), Voice::Neutral);
        assert_eq!(m.tone(), Tone::Neutral);
        assert_eq!(m.warmth(), Warmth::Neutral);
        assert_eq!(m.format(), Format::Prose);
        assert_eq!(m.accuracy(), Accuracy::Medium);
        assert_eq!(m.urgency(), Urgency::Normal);
    }

    #[test]
    fn test_field_extraction() {
        // Casual(0x8000) + Empathetic(0x2000) + Neutral warmth(0x0400) + Medium accuracy(0x0040)
        let m = Modifier::from_u16(0xA440);
        assert_eq!(m.voice(), Voice::Casual);
        assert_eq!(m.tone(), Tone::Empathetic);
        assert_eq!(m.warmth(), Warmth::Neutral);
        assert_eq!(m.accuracy(), Accuracy::Medium);
    }

    #[test]
    fn test_field_setting() {
        let m = Modifier::default()
            .with_voice(Voice::Formal)
            .with_tone(Tone::Empathetic)
            .with_warmth(Warmth::VeryWarm)
            .with_format(Format::Bulleted);

        assert_eq!(m.voice(), Voice::Formal);
        assert_eq!(m.tone(), Tone::Empathetic);
        assert_eq!(m.warmth(), Warmth::VeryWarm);
        assert_eq!(m.format(), Format::Bulleted);
    }

    #[test]
    fn test_crisis_modifier() {
        let m = Modifier::crisis();
        assert_eq!(m.tone(), Tone::Empathetic);
        assert_eq!(m.warmth(), Warmth::VeryWarm);
        assert_eq!(m.urgency(), Urgency::High);
        assert_eq!(m.accuracy(), Accuracy::High);
    }

    #[test]
    fn test_preset_modifiers() {
        let pro = Modifier::professional();
        assert_eq!(pro.voice(), Voice::Formal);

        let friendly = Modifier::friendly();
        assert_eq!(friendly.voice(), Voice::Casual);
        assert_eq!(friendly.warmth(), Warmth::Warm);
    }

    #[test]
    fn test_serialization() {
        let modifier = Modifier::crisis();
        let json = serde_json::to_string(&modifier).unwrap();
        let deserialized: Modifier = serde_json::from_str(&json).unwrap();
        assert_eq!(modifier, deserialized);
    }
}
