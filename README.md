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

## TRM Integration

This crate is designed for use with TinyRecursiveModels (TRMs) that output opcodes directly.

The factored prediction approach uses 3 heads:

- ACT head → Action code
- SUBJ head → Subject code
- MOD head → Modifier flags

This allows small models (~148K params) to achieve 99%+ accuracy on opcode prediction.

## Related Crates

- **frame-catalog**: Vector store and RAG system

## License

MIT
