//! Extended instruction format with argument payloads
//!
//! Extends the base 6-byte opcode with additional argument data:
//!
//! ```text
//! [BASE:6 bytes][PAYLOAD_TYPE:1 byte][PAYLOAD:N bytes]
//! ```
//!
//! Payload types:
//! - 0x00: None (base instruction only)
//! - 0x01: CalcArgs (17 bytes: [OP:1][A:8][B:8])
//! - 0x02: TimeArgs (14 bytes: [REF:8][DELTA:4][UNIT:1][TZ:1])
//!
//! This format allows opcodes to be self-contained, carrying all data
//! needed for execution without external context.

use crate::{Instruction, InstructionError, INSTRUCTION_SIZE};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Payload type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PayloadType {
    /// No payload (base instruction only)
    None = 0x00,
    /// Calculator arguments: [OP:1][A:8][B:8] = 17 bytes
    Calc = 0x01,
    /// Time arguments: [REF:8][DELTA:4][UNIT:1][TZ:1] = 14 bytes
    Time = 0x02,
}

impl PayloadType {
    /// Parse from byte
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x00 => Some(PayloadType::None),
            0x01 => Some(PayloadType::Calc),
            0x02 => Some(PayloadType::Time),
            _ => None,
        }
    }

    /// Convert to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Get payload size in bytes
    pub fn payload_size(self) -> usize {
        match self {
            PayloadType::None => 0,
            PayloadType::Calc => 17, // [OP:1][A:8][B:8]
            PayloadType::Time => 14, // [REF:8][DELTA:4][UNIT:1][TZ:1]
        }
    }

    /// Get total extended instruction size (6 base + 1 type + N payload)
    pub fn total_size(self) -> usize {
        INSTRUCTION_SIZE + 1 + self.payload_size()
    }
}

/// Arithmetic operation type (matches frame-interpreter CalcArgs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Op {
    Add = 0x2B,  // '+'
    Sub = 0x2D,  // '-'
    Mul = 0x2A,  // '*'
    Div = 0x2F,  // '/'
    Mod = 0x25,  // '%'
    Pow = 0x5E,  // '^'
    Sqrt = 0x53, // 'S'
}

impl Op {
    /// Parse from byte
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x2B => Some(Op::Add),
            0x2D => Some(Op::Sub),
            0x2A => Some(Op::Mul),
            0x2F => Some(Op::Div),
            0x25 => Some(Op::Mod),
            0x5E => Some(Op::Pow),
            0x53 => Some(Op::Sqrt),
            _ => None,
        }
    }

    /// Convert to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Symbol for display
    pub fn symbol(self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Mod => "%",
            Op::Pow => "^",
            Op::Sqrt => "sqrt",
        }
    }
}

/// Calculator arguments payload
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CalcPayload {
    pub op: Op,
    pub a: f64,
    pub b: f64,
}

impl CalcPayload {
    /// Create new calc payload
    pub fn new(op: Op, a: f64, b: f64) -> Self {
        Self { op, a, b }
    }

    /// Create unary operation (sqrt, etc.)
    pub fn unary(op: Op, a: f64) -> Self {
        Self { op, a, b: 0.0 }
    }

    /// Serialize to bytes: [OP:1][A:8][B:8] = 17 bytes
    pub fn to_bytes(&self) -> [u8; 17] {
        let mut bytes = [0u8; 17];
        bytes[0] = self.op.to_byte();
        bytes[1..9].copy_from_slice(&self.a.to_be_bytes());
        bytes[9..17].copy_from_slice(&self.b.to_be_bytes());
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 17 {
            return None;
        }
        let op = Op::from_byte(bytes[0])?;
        let a = f64::from_be_bytes(bytes[1..9].try_into().ok()?);
        let b = f64::from_be_bytes(bytes[9..17].try_into().ok()?);
        Some(Self { op, a, b })
    }
}

impl fmt::Display for CalcPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(self.op, Op::Sqrt) {
            write!(f, "{}({})", self.op.symbol(), self.a)
        } else {
            write!(f, "{} {} {}", self.a, self.op.symbol(), self.b)
        }
    }
}

/// Time unit for temporal calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum TimeUnit {
    Second = 0,
    Minute = 1,
    Hour = 2,
    Day = 3,
    Week = 4,
    Month = 5,
    Year = 6,
}

impl TimeUnit {
    /// Parse from byte
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(TimeUnit::Second),
            1 => Some(TimeUnit::Minute),
            2 => Some(TimeUnit::Hour),
            3 => Some(TimeUnit::Day),
            4 => Some(TimeUnit::Week),
            5 => Some(TimeUnit::Month),
            6 => Some(TimeUnit::Year),
            _ => None,
        }
    }

    /// Convert to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Seconds per unit
    pub fn seconds(self) -> i64 {
        match self {
            TimeUnit::Second => 1,
            TimeUnit::Minute => 60,
            TimeUnit::Hour => 3600,
            TimeUnit::Day => 86400,
            TimeUnit::Week => 604800,
            TimeUnit::Month => 2592000,  // ~30 days
            TimeUnit::Year => 31536000,  // 365 days
        }
    }

    /// Display name
    pub fn name(self) -> &'static str {
        match self {
            TimeUnit::Second => "second",
            TimeUnit::Minute => "minute",
            TimeUnit::Hour => "hour",
            TimeUnit::Day => "day",
            TimeUnit::Week => "week",
            TimeUnit::Month => "month",
            TimeUnit::Year => "year",
        }
    }
}

/// Time arguments payload
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimePayload {
    /// Reference timestamp (Unix seconds)
    pub reference: i64,
    /// Delta value (positive = future, negative = past)
    pub delta: i32,
    /// Unit of the delta
    pub unit: TimeUnit,
    /// Timezone offset in hours (-12 to +14)
    pub tz_offset: i8,
}

impl TimePayload {
    /// Create time payload for "now"
    pub fn now() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let reference = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        Self {
            reference,
            delta: 0,
            unit: TimeUnit::Second,
            tz_offset: 0,
        }
    }

    /// Create with specific reference
    pub fn at(reference: i64) -> Self {
        Self {
            reference,
            delta: 0,
            unit: TimeUnit::Second,
            tz_offset: 0,
        }
    }

    /// Create with delta from reference
    pub fn with_delta(reference: i64, delta: i32, unit: TimeUnit) -> Self {
        Self {
            reference,
            delta,
            unit,
            tz_offset: 0,
        }
    }

    /// Set timezone offset
    pub fn with_tz(mut self, offset: i8) -> Self {
        self.tz_offset = offset;
        self
    }

    /// Calculate target timestamp
    pub fn target_timestamp(&self) -> i64 {
        let delta_seconds = (self.delta as i64) * self.unit.seconds();
        let tz_seconds = (self.tz_offset as i64) * 3600;
        self.reference + delta_seconds + tz_seconds
    }

    /// Serialize to bytes: [REF:8][DELTA:4][UNIT:1][TZ:1] = 14 bytes
    pub fn to_bytes(&self) -> [u8; 14] {
        let mut bytes = [0u8; 14];
        bytes[0..8].copy_from_slice(&self.reference.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.delta.to_be_bytes());
        bytes[12] = self.unit.to_byte();
        bytes[13] = self.tz_offset as u8;
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 14 {
            return None;
        }
        let reference = i64::from_be_bytes(bytes[0..8].try_into().ok()?);
        let delta = i32::from_be_bytes(bytes[8..12].try_into().ok()?);
        let unit = TimeUnit::from_byte(bytes[12])?;
        let tz_offset = bytes[13] as i8;
        Some(Self {
            reference,
            delta,
            unit,
            tz_offset,
        })
    }
}

/// Payload variants for extended instructions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Payload {
    None,
    Calc(CalcPayload),
    Time(TimePayload),
}

impl Payload {
    /// Get payload type
    pub fn payload_type(&self) -> PayloadType {
        match self {
            Payload::None => PayloadType::None,
            Payload::Calc(_) => PayloadType::Calc,
            Payload::Time(_) => PayloadType::Time,
        }
    }

    /// Serialize payload to bytes (not including type byte)
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Payload::None => Vec::new(),
            Payload::Calc(c) => c.to_bytes().to_vec(),
            Payload::Time(t) => t.to_bytes().to_vec(),
        }
    }

    /// Parse payload from type and bytes
    pub fn from_bytes(payload_type: PayloadType, bytes: &[u8]) -> Option<Self> {
        match payload_type {
            PayloadType::None => Some(Payload::None),
            PayloadType::Calc => CalcPayload::from_bytes(bytes).map(Payload::Calc),
            PayloadType::Time => TimePayload::from_bytes(bytes).map(Payload::Time),
        }
    }
}

/// Extended instruction with argument payload
///
/// Format:
/// ```text
/// [BASE:6 bytes][PAYLOAD_TYPE:1 byte][PAYLOAD:N bytes]
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedInstruction {
    /// Base instruction
    pub base: Instruction,
    /// Argument payload
    pub payload: Payload,
}

impl ExtendedInstruction {
    /// Create extended instruction with no payload
    pub fn new(base: Instruction) -> Self {
        Self {
            base,
            payload: Payload::None,
        }
    }

    /// Create extended instruction with calc payload
    pub fn with_calc(base: Instruction, calc: CalcPayload) -> Self {
        Self {
            base,
            payload: Payload::Calc(calc),
        }
    }

    /// Create extended instruction with time payload
    pub fn with_time(base: Instruction, time: TimePayload) -> Self {
        Self {
            base,
            payload: Payload::Time(time),
        }
    }

    /// Get total byte size
    pub fn byte_size(&self) -> usize {
        self.payload.payload_type().total_size()
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.byte_size());
        bytes.extend_from_slice(&self.base.to_bytes());
        bytes.push(self.payload.payload_type().to_byte());
        bytes.extend_from_slice(&self.payload.to_bytes());
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InstructionError> {
        if bytes.len() < INSTRUCTION_SIZE + 1 {
            return Err(InstructionError::InvalidLength {
                actual: bytes.len(),
                expected_multiple_of: INSTRUCTION_SIZE + 1,
            });
        }

        let base = Instruction::parse_one(&bytes[..INSTRUCTION_SIZE])?;
        let payload_type = PayloadType::from_byte(bytes[INSTRUCTION_SIZE]).ok_or(
            InstructionError::InvalidOpcodeString(format!(
                "Unknown payload type: 0x{:02X}",
                bytes[INSTRUCTION_SIZE]
            )),
        )?;

        let expected_size = payload_type.total_size();
        if bytes.len() < expected_size {
            return Err(InstructionError::InvalidLength {
                actual: bytes.len(),
                expected_multiple_of: expected_size,
            });
        }

        let payload = Payload::from_bytes(payload_type, &bytes[INSTRUCTION_SIZE + 1..]).ok_or(
            InstructionError::InvalidOpcodeString("Failed to parse payload".to_string()),
        )?;

        Ok(Self { base, payload })
    }

    /// Get as calc payload if present
    pub fn as_calc(&self) -> Option<&CalcPayload> {
        match &self.payload {
            Payload::Calc(c) => Some(c),
            _ => None,
        }
    }

    /// Get as time payload if present
    pub fn as_time(&self) -> Option<&TimePayload> {
        match &self.payload {
            Payload::Time(t) => Some(t),
            _ => None,
        }
    }
}

impl fmt::Display for ExtendedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base)?;
        match &self.payload {
            Payload::None => Ok(()),
            Payload::Calc(c) => write!(f, " + {}", c),
            Payload::Time(t) => write!(f, " @ {}", t.target_timestamp()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Action, Modifier, Subject};

    #[test]
    fn test_calc_payload_roundtrip() {
        let calc = CalcPayload::new(Op::Add, 15.0, 7.0);
        let bytes = calc.to_bytes();
        let parsed = CalcPayload::from_bytes(&bytes).unwrap();
        assert_eq!(calc, parsed);
    }

    #[test]
    fn test_time_payload_roundtrip() {
        let time = TimePayload::with_delta(1000000, -5, TimeUnit::Minute).with_tz(-8);
        let bytes = time.to_bytes();
        let parsed = TimePayload::from_bytes(&bytes).unwrap();
        assert_eq!(time, parsed);
    }

    #[test]
    fn test_extended_instruction_no_payload() {
        let base = Instruction::new(Action::RESPOND, Subject::TIME, Modifier::default());
        let ext = ExtendedInstruction::new(base);

        let bytes = ext.to_bytes();
        assert_eq!(bytes.len(), 7); // 6 + 1

        let parsed = ExtendedInstruction::from_bytes(&bytes).unwrap();
        assert_eq!(ext.base, parsed.base);
        assert!(matches!(parsed.payload, Payload::None));
    }

    #[test]
    fn test_extended_instruction_calc() {
        let base = Instruction::new(Action::CALCULATE, Subject::NUMBER, Modifier::default());
        let calc = CalcPayload::new(Op::Mul, 6.0, 7.0);
        let ext = ExtendedInstruction::with_calc(base, calc);

        let bytes = ext.to_bytes();
        assert_eq!(bytes.len(), 24); // 6 + 1 + 17

        let parsed = ExtendedInstruction::from_bytes(&bytes).unwrap();
        assert_eq!(ext.base, parsed.base);
        assert_eq!(parsed.as_calc().unwrap(), &calc);
    }

    #[test]
    fn test_extended_instruction_time() {
        let base = Instruction::new(Action::RESPOND, Subject::TIME, Modifier::default());
        let time = TimePayload::with_delta(1735300000, 3, TimeUnit::Hour);
        let ext = ExtendedInstruction::with_time(base, time);

        let bytes = ext.to_bytes();
        assert_eq!(bytes.len(), 21); // 6 + 1 + 14

        let parsed = ExtendedInstruction::from_bytes(&bytes).unwrap();
        assert_eq!(ext.base, parsed.base);
        assert_eq!(parsed.as_time().unwrap(), &time);
    }

    #[test]
    fn test_payload_type_sizes() {
        assert_eq!(PayloadType::None.payload_size(), 0);
        assert_eq!(PayloadType::Calc.payload_size(), 17);
        assert_eq!(PayloadType::Time.payload_size(), 14);

        assert_eq!(PayloadType::None.total_size(), 7);
        assert_eq!(PayloadType::Calc.total_size(), 24);
        assert_eq!(PayloadType::Time.total_size(), 21);
    }

    #[test]
    fn test_calc_payload_display() {
        let add = CalcPayload::new(Op::Add, 15.0, 7.0);
        assert_eq!(format!("{}", add), "15 + 7");

        let sqrt = CalcPayload::unary(Op::Sqrt, 144.0);
        assert_eq!(format!("{}", sqrt), "sqrt(144)");
    }

    #[test]
    fn test_target_timestamp() {
        let time = TimePayload {
            reference: 1000000,
            delta: 5,
            unit: TimeUnit::Minute,
            tz_offset: 0,
        };
        assert_eq!(time.target_timestamp(), 1000300); // +300 seconds

        let time_tz = TimePayload {
            reference: 1000000,
            delta: 0,
            unit: TimeUnit::Second,
            tz_offset: -8,
        };
        assert_eq!(time_tz.target_timestamp(), 1000000 - 8 * 3600);
    }
}
