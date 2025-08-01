markov
Markov model implementation on sequence of bytes.

Quick Start
$ rustc -C opt-level=3 markov.rs
$ ./markov train ./LICENSE model
$ ./markov gen model



I'll explain the `markov.rs` file in detail. This is a Markov chain text generator written in Rust that can train models from text data and generate new text based on learned patterns.

## Overview

This program implements a Markov chain text generator with three main operations:
- **train**: Create a model from input text
- **gen**: Generate text using a trained model  
- **stats**: Show statistics about a model

## Detailed Component Breakdown

### 1. Linear Congruential Generator (LCG) - Lines 8-26

```rust
struct LCG {
    state: u64
}
```

This implements a pseudo-random number generator:
- **Purpose**: Provides deterministic randomness for text generation
- **Algorithm**: Uses Donald Knuth's MMIX constants for good randomness properties
- **Method**: `random_u32()` generates 32-bit random numbers using the formula: `state = (state * a + c) mod 2^64`

### 2. Frequency Distribution (Freq) - Lines 28-110

```rust
struct Freq {
    tokens: Vec<(u8, u32)>,
}
```

This tracks how often each byte follows a given context:
- **Data Structure**: Vector of (byte, count) pairs
- **Key Methods**:
  - `push(x)`: Increments count for byte `x`, or adds it with count 1
  - `random(lcg)`: Randomly selects a byte based on weighted probabilities
  - `write_to()` / `read_from()`: Binary serialization for model persistence

**Random Selection Algorithm (Lines 75-89)**:
1. Calculate total frequency sum
2. Generate random index in range [0, sum)
3. Walk through tokens until cumulative sum exceeds index
4. Return the corresponding byte

### 3. Binary I/O Helpers - Lines 33-49

```rust
fn read_u8(r: &mut impl io::Read) -> io::Result<u8>
fn read_u32(r: &mut impl io::Read) -> io::Result<u32>  
fn read_u64(r: &mut impl io::Read) -> io::Result<u64>
```

These functions read little-endian encoded integers from binary streams, used for model serialization.

### 4. Markov Model (Model) - Lines 112-160

```rust
struct Model {
    model: HashMap<u64, Freq>,
}
```

This is the core Markov chain implementation:
- **Key Concept**: Maps 8-byte contexts to frequency distributions
- **Context Window**: Uses previous 8 bytes as context for predicting next byte
- **Key Methods**:
  - `push(context, next)`: Records that `next` byte follows `context`
  - `random(context, lcg)`: Predicts next byte given context
  - `write_to()` / `read_from()`: Model persistence

### 5. Data Slicer (Slicer) - Lines 162-189

```rust
struct Slicer {
    bytes: Vec<u8>,
    window: u64,
    cursor: usize,
}
```

This processes input data into (context, next_byte) pairs:
- **Sliding Window**: Maintains 8-byte context window
- **Iterator Implementation**: Yields (context, next_byte) tuples
- **Window Update**: Shifts left by 8 bits and adds new byte: `window = (window << 8) | next_byte`

### 6. Main Program Logic - Lines 203-327

The program handles three subcommands:

#### **Training Mode** (`train <INPUT> <OUTPUT>`):
1. Read input file as bytes
2. Process through Slicer to extract (context, next_byte) pairs
3. Build frequency distributions for each context
4. Serialize model to binary output file

#### **Generation Mode** (`gen <FILE> [-l <LIMIT>]`):
1. Load pre-trained model from file
2. Start with empty context (0)
3. Repeatedly:
   - Query model for next byte given current context
   - Output the byte
   - Update context with new byte
   - Stop at limit or when no prediction available

#### **Statistics Mode** (`stats <FILE>`):
1. Train model from input file
2. Calculate and display:
   - Total number of unique contexts
   - Maximum branching factor (most possible next bytes for any context)
   - Average branching factor across all contexts

## Key Algorithms

### Context Management
```rust
fn context_push(context: &mut u64, x: u8) {
    *context = ((*context)<<8)|(x as u64);
}
```
This maintains a sliding 8-byte window by shifting left and adding new byte.

### Weighted Random Selection
The `Freq::random()` method implements weighted selection:
1. Sum all frequencies
2. Pick random number in [0, sum)
3. Find which token's cumulative frequency contains this number

## Data Flow

1. **Training**: Text → Slicer → (context, byte) pairs → Model → Binary file
2. **Generation**: Binary file → Model → Random walk → Generated text
3. **Stats**: Text → Model → Statistical analysis

## File Format

The binary model format stores:
- Number of contexts (u64)
- For each context:
  - Context value (u64)
  - Number of possible next bytes (u8)
  - For each next byte: byte value (u8) + frequency (u32)

This implementation creates a character-level Markov chain with 8 bytes of context, making it capable of learning and reproducing complex patterns in text data.
