# Frame ISA

**SAM Instruction Set Architecture** - Zero-dependency 6-byte opcode definitions for AI systems.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Frame ISA defines a compact instruction format for AI output:

```
[ACT:2 bytes][SUBJ:2 bytes][MOD:2 bytes] = 6 bytes total (big-endian)
```

- **ACT (Action)**: What operation to perform (GREET, RESPOND, CALCULATE, etc.)
- **SUBJ (Subject)**: The topic or entity (TIME, USER, WEATHER, RAG reference, etc.)
- **MOD (Modifier)**: Style flags (voice, tone, warmth, format, urgency, etc.)

## Installation

```toml
[dependencies]
frame-isa = "0.1"
```

## Usage

```rust
use frame_isa::{Action, Subject, Modifier, Instruction};

// Create an instruction
let instr = Instruction::new(
    Action::RESPOND,
    Subject::TIME,
    Modifier::default(),
);

// Serialize to bytes
let bytes = instr.to_bytes();
assert_eq!(bytes.len(), 6);

// Parse from bytes
let parsed = Instruction::parse_one(&bytes).unwrap();
assert_eq!(instr, parsed);
```

### Using the Builder

```rust
use frame_isa::{Action, Subject, InstructionBuilder, Voice, Tone};

let instr = InstructionBuilder::new(Action::GREET)
    .subject(Subject::USER)
    .voice(Voice::Casual)
    .tone(Tone::Positive)
    .build();
```

### Preset Modifiers

```rust
use frame_isa::Modifier;

// For crisis situations
let crisis = Modifier::crisis();  // Empathetic, warm, high urgency

// For professional contexts
let pro = Modifier::professional();  // Formal, neutral warmth

// For friendly interactions
let friendly = Modifier::friendly();  // Casual, warm, positive
```

## Opcode Categories

### Actions (ACT)

| Range  | Category  | Examples                               |
| ------ | --------- | -------------------------------------- |
| 0x00xx | System    | NOP, HALT, ERROR, STATUS               |
| 0x01xx | Response  | GREET, CONFIRM, DENY, EXPLAIN, RESPOND |
| 0x02xx | Query     | ASK, REQUEST, SEARCH, RETRIEVE         |
| 0x03xx | Knowledge | DEFINE, DESCRIBE, COMPARE, SUMMARIZE   |
| 0x04xx | Skill     | CALCULATE, SET_TIMER, KNOWLEDGE_SEARCH |
| 0x05xx | Emotion   | EMPATHY, CONCERN, ENCOURAGEMENT        |
| 0x06xx | Template  | TEMPLATE_LOAD, TEMPLATE_FILL           |
| 0x07xx | Chain     | CHAIN, FORK, MERGE                     |

### Subjects (SUBJ)

| Range  | Category     | Examples                       |
| ------ | ------------ | ------------------------------ |
| 0x00xx | System       | NULL, SELF, USER, CONTEXT      |
| 0x01xx | Common       | WEATHER, TIME, DATE, SCHEDULE  |
| 0x02xx | Math/Science | NUMBER, EQUATION, PHYSICS      |
| 0x03xx | Technology   | COMPUTER, SOFTWARE, AI, API    |
| 0x04xx | Knowledge    | DOCUMENTATION, CONCEPT         |
| 0x05xx | Emotions     | FEELINGS, STRESS, ANXIETY      |
| 0x06xx | TRM Refs     | References to other TRM models |
| 0xE0xx | RAG Refs     | Dynamic document lookups       |

### Modifier Bit Layout

```
Bit:  15  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0
      [--VOICE--] [--TONE--] [-WARM-] [--FORMAT--] [ACCURACY] [URGENCY]
```

| Field    | Bits  | Values                                  |
| -------- | ----- | --------------------------------------- |
| Voice    | 15-14 | Neutral, Formal, Casual, Technical      |
| Tone     | 13-12 | Neutral, Positive, Empathetic, Cautious |
| Warmth   | 11-10 | Cold, Neutral, Warm, VeryWarm           |
| Format   | 9-8   | Prose, Bulleted, Numbered, Structured   |
| Accuracy | 7-6   | Low, Medium, High, Verified             |
| Urgency  | 5-4   | Low, Normal, High, Critical             |

## Extended Instructions

For instructions that require operands (calculations, time queries), use `ExtendedInstruction`:

```
[BASE:6 bytes][PAYLOAD_TYPE:1 byte][PAYLOAD:N bytes]
```

### Payload Types

| Type | ID   | Size | Description                                                   |
| ---- | ---- | ---- | ------------------------------------------------------------- |
| None | 0x00 | 0    | Base instruction only (7 bytes total)                         |
| Calc | 0x01 | 17   | `[OP:1][A:8][B:8]` - Calculator args (24 bytes total)         |
| Time | 0x02 | 14   | `[REF:8][DELTA:4][UNIT:1][TZ:1]` - Time args (21 bytes total) |

### Calculator Payload

```rust
use frame_isa::{Action, Subject, Modifier, Instruction, ExtendedInstruction, CalcPayload, Op};

// "15 + 7" -> CALCULATE instruction with args
let base = Instruction::new(Action::CALCULATE, Subject::NUMBER, Modifier::default());
let calc = CalcPayload::new(Op::Add, 15.0, 7.0);
let ext = ExtendedInstruction::with_calc(base, calc);

// Serialize: 24 bytes total
let bytes = ext.to_bytes();
assert_eq!(bytes.len(), 24);

// Parse back
let parsed = ExtendedInstruction::from_bytes(&bytes).unwrap();
assert_eq!(parsed.as_calc().unwrap().a, 15.0);
```

### Time Payload

```rust
use frame_isa::{Action, Subject, Modifier, Instruction, ExtendedInstruction, TimePayload, TimeUnit};

// "what time will it be in 3 hours"
let base = Instruction::new(Action::RESPOND, Subject::TIME, Modifier::default());
let time = TimePayload::with_delta(1735300000, 3, TimeUnit::Hour);
let ext = ExtendedInstruction::with_time(base, time);

// Serialize: 21 bytes total
let bytes = ext.to_bytes();
assert_eq!(bytes.len(), 21);
```

### Operations

| Op   | Byte | Symbol | Description         |
| ---- | ---- | ------ | ------------------- |
| Add  | 0x2B | +      | Addition            |
| Sub  | 0x2D | -      | Subtraction         |
| Mul  | 0x2A | *      | Multiplication      |
| Div  | 0x2F | /      | Division            |
| Mod  | 0x25 | %      | Modulo              |
| Pow  | 0x5E | ^      | Power               |
| Sqrt | 0x53 | sqrt   | Square root (unary) |

## TRM Integration

This crate is designed for use with TinyRecursiveModels (TRMs) that output opcodes directly.

### Factored Prediction

The factored prediction approach uses multiple heads:

**Base Opcode (3 heads):**

- ACT head → Action code
- SUBJ head → Subject code
- MOD head → Modifier flags

**Extended Args (3 additional heads for CalcArgsModel):**

- Op head → Operation type (7 classes)
- A head → First operand (regression or pointer)
- B head → Second operand (regression or pointer)

### MicroChip Architecture

For complex domains like math, multiple specialized MicroChips can work together:

```
┌─────────────────────────────────────────────────┐
│                  IntelliChip                    │
│         (Router + Context Manager)              │
└────────────────────┬────────────────────────────┘
                     │
       ┌─────────────┼─────────────┐
       ▼             ▼             ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│ OpClassifier│ │ NumExtractor│ │ UnitParser  │
│  MicroChip  │ │  MicroChip  │ │  MicroChip  │
│   (~50K)    │ │   (~50K)    │ │   (~30K)    │
└─────────────┘ └─────────────┘ └─────────────┘
       │             │             │
       └─────────────┼─────────────┘
                     ▼
              ExtendedInstruction
```

Each MicroChip is tiny (~30-50K params) and specialized. The IntelliChip orchestrates them to produce complete extended instructions.

## Related Crates

- **frame-interpreter**: Execute instructions and return typed results
- **frame-catalog**: Vector store and RAG system

## License

MIT
