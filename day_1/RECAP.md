# Advent of Code Day 1 - Rust Solution Recap

## Problem Overview

**Challenge**: Count how many times a dial (0-99) points at 0 after each rotation in a sequence.

**Key Requirements**:
- Dial starts at position 50
- Rotations: `L` (left, decreasing) or `R` (right, increasing) followed by a distance
- Dial wraps around: 0-99 (circular)
- Count occurrences where position equals 0 after any rotation

**Example**:
```
Start: 50
L68 → 82
L30 → 52
R48 → 0  ← count this!
L5 → 95
...
Result: 3 times at position 0
```

---

## Solution Architecture

### Function Breakdown

1. **`read_input_file`** - File I/O with error handling
2. **`handle_file_error`** - Specific error message handling
3. **`parse_rotation`** - String parsing with validation
4. **`apply_rotation`** - Modular arithmetic for dial rotation
5. **`solve_puzzle`** - Main algorithm logic
6. **`main`** - Program entry point and orchestration

---

## Rust Concepts Explained

### 1. Error Handling with `Result<T, E>`

#### What is `Result`?

`Result` is Rust's way of handling operations that can succeed or fail. It's an enum with two variants:

```rust
enum Result<T, E> {
    Ok(T),   // Success case - contains the value
    Err(E),  // Error case - contains error information
}
```

#### Why Use `Result`?

- **Explicit error handling**: Forces you to handle errors
- **No hidden failures**: Can't ignore errors accidentally
- **Type safety**: Compiler ensures you handle both cases

#### Example from Our Code:

```rust
fn read_input_file(filename: &str) -> Result<String, Error> {
    read_to_string(filename)  // Returns Result<String, Error>
}
```

**Breakdown**:
- `Result<String, Error>` means:
  - Success: Returns `Ok(String)` containing file contents
  - Failure: Returns `Err(Error)` containing error details
- `read_to_string` can fail if file doesn't exist or can't be read

---

### 2. Pattern Matching with `match`

#### What is `match`?

`match` is Rust's exhaustive pattern matching construct. It's like `switch` but:
- **Exhaustive**: Must handle all possible cases
- **Pattern matching**: Can match on structure, not just values
- **Expression**: Returns a value

#### Syntax:

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_case,  // Catch-all
}
```

#### Example from Our Code:

```rust
let input = match read_input_file(filename) {
    Ok(input) => input,           // If successful, extract the String
    Err(e) => {                   // If error, handle it
        handle_file_error(e, filename);
    }
};
```

**Breakdown**:
- `Ok(input)` pattern: Matches success case, binds value to `input`
- `Err(e)` pattern: Matches error case, binds error to `e`
- Each arm returns a value (or in this case, `handle_file_error` never returns)

---

### 3. The Never Type `!`

#### What is `!`?

The never type `!` represents a value that never exists. Functions that never return normally use this.

#### When is it used?

- Functions that always panic
- Functions that always exit the program
- Infinite loops

#### Example from Our Code:

```rust
fn handle_file_error(e: Error, filename: &str) -> ! {
    // ... error handling ...
    std::process::exit(1);  // Never returns
}
```

**Why `!` matters**:
- Tells the compiler: "This function never returns normally"
- Allows the compiler to understand control flow
- In `match`, if one arm returns `!`, the other arm's return type is used

```rust
let input = match read_input_file(filename) {
    Ok(input) => input,        // Returns String
    Err(e) => handle_file_error(e, filename),  // Returns ! (never)
};
// Compiler knows: input is always String here
```

---

### 4. String Slices `&str` vs Owned Strings `String`

#### `&str` (String Slice)

- **Borrowed reference**: Doesn't own the data
- **Immutable**: Can't modify
- **Cheap**: Just a pointer + length
- **Use for**: Function parameters when you don't need ownership

#### `String` (Owned String)

- **Owns the data**: Allocates memory
- **Mutable**: Can grow/shrink
- **More expensive**: Heap allocation
- **Use for**: When you need to own or modify the string

#### Example from Our Code:

```rust
fn parse_rotation(line: &str) -> Result<(char, i32), String> {
    // line is &str - borrowed, can't modify
    // Return type uses String for error messages (owned)
}
```

**Why `&str` for parameters?**
- More flexible: accepts both `&str` and `&String`
- More efficient: no copying
- Clear intent: "I just need to read this"

---

### 5. Pattern Matching with Bindings (`@`)

#### The `@` Operator

Allows you to match a pattern AND bind the matched value to a variable.

#### Syntax:

```rust
pattern @ value => use_both_pattern_and_value
```

#### Example from Our Code:

```rust
let direction = match line.chars().next() {
    Some(d @ 'L') | Some(d @ 'R') => d,
    _ => return Err(format!("Invalid direction in '{}'", line)),
};
```

**Breakdown**:
- `Some(d @ 'L')`: Matches `Some('L')` AND binds the char to `d`
- `Some(d @ 'R')`: Matches `Some('R')` AND binds the char to `d`
- `|`: Combines patterns (matches either)
- Result: `d` contains the matched character ('L' or 'R')

**Why not just `line.chars().next().unwrap()`?**
- `unwrap()` would panic on `None` (empty string)
- We want to handle errors gracefully
- Pattern matching is explicit and safe

---

### 6. String Parsing with `.parse()`

#### What is `.parse()`?

Converts a string slice to another type. Returns `Result<T, ParseError>`.

#### Syntax:

```rust
let number: i32 = "123".parse().unwrap();  // Simple (panics on error)
let number = "123".parse::<i32>().unwrap(); // Explicit type
```

#### Example from Our Code:

```rust
let distance = match line[1..].parse::<i32>() {
    Ok(d) => d,  // Success: use the parsed number
    Err(e) => return Err(format!("Invalid number in '{}': {}", line, e)),
};
```

**Breakdown**:
- `line[1..]`: String slice from index 1 to end (skips first character)
- `.parse::<i32>()`: Parse as 32-bit integer
- `match`: Handle both success and failure
- `Err(e)`: Extract error details for user-friendly message

---

### 7. String Slicing

#### Syntax:

```rust
&string[start..end]    // From start (inclusive) to end (exclusive)
&string[start..]       // From start to end of string
&string[..end]         // From beginning to end (exclusive)
&string[..]            // Entire string (as slice)
```

#### Example from Our Code:

```rust
let distance = line[1..].parse::<i32>();
//     ^^^^^^
//     Gets everything after first character
//     "R17" → "17"
```

**Important**: String slicing in Rust uses byte indices, not character indices. For ASCII (like our input), this works fine. For Unicode, use `.chars()`.

---

### 8. Modular Arithmetic

#### The Problem

Dial wraps around: 0-99 (100 positions). Need to handle:
- Right rotation: `(position + distance) % 100`
- Left rotation: `(position - distance) % 100` can be negative!

#### Solution:

```rust
fn apply_rotation(position: i32, direction: char, distance: i32) -> i32 {
    match direction {
        'R' => (position + distance) % 100,
        'L' => (position - distance + 100) % 100,
        _ => position,
    }
}
```

**Breakdown**:
- **Right**: `(position + distance) % 100`
  - Example: `(99 + 5) % 100 = 104 % 100 = 4` ✓
  
- **Left**: `(position - distance + 100) % 100`
  - The `+ 100` ensures result is positive before modulo
  - Example: `(5 - 10 + 100) % 100 = 95 % 100 = 95` ✓
  - Without `+ 100`: `(5 - 10) % 100 = -5 % 100` (implementation-defined, may be negative!)

**Why `+ 100`?**
- Ensures the value is in range [0, 199] before modulo
- After `% 100`, always in range [0, 99]
- Works for any negative result

---

### 9. Mutability with `let mut`

#### Immutable by Default

Rust variables are immutable by default:

```rust
let x = 5;
x = 6;  // ERROR: cannot assign twice to immutable variable
```

#### Making Variables Mutable

Use `mut` keyword:

```rust
let mut x = 5;
x = 6;  // OK: variable is mutable
```

#### Example from Our Code:

```rust
let mut position = 50;  // Can change
let mut count = 0;      // Can change
```

**Why mutability matters**:
- Safety: Immutability prevents accidental changes
- Clarity: `mut` signals intent to modify
- Performance: Compiler can optimize immutable values better

---

### 10. Iterator Methods: `.lines()` and `.chars()`

#### `.lines()`

Returns an iterator over lines in a string:

```rust
for line in input.lines() {
    // line is &str (each line)
}
```

**Characteristics**:
- Splits on newline characters (`\n` or `\r\n`)
- Each iteration gives a `&str` slice
- Handles empty lines (returns empty string `""`)

#### `.chars()`

Returns an iterator over Unicode scalar values (characters):

```rust
for ch in "hello".chars() {
    // ch is char
}
```

**In our code**:
```rust
line.chars().next()  // Gets first character as Option<char>
```

**Why `.next()`?**
- Iterators are lazy: don't compute until needed
- `.next()` gets the first item
- Returns `Option<T>`: `Some(value)` or `None`

---

### 11. String Methods: `.trim()` and `.is_empty()`

#### `.trim()`

Removes leading and trailing whitespace:

```rust
let s = "  hello  ";
s.trim();  // "hello"
```

#### `.is_empty()`

Checks if string has zero length:

```rust
"".is_empty()      // true
"hello".is_empty() // false
```

#### Example from Our Code:

```rust
let line = line.trim();  // Remove whitespace

if line.is_empty() {
    continue;  // Skip empty lines
}
```

**Why trim first?**
- Input files may have trailing newlines/spaces
- Empty lines should be skipped
- Cleaner data processing

---

### 12. Control Flow: `continue` and Early Returns

#### `continue`

Skips to next iteration of loop:

```rust
for item in items {
    if should_skip(item) {
        continue;  // Skip to next item
    }
    // Process item
}
```

#### Early Return

Return from function before reaching end:

```rust
fn parse_rotation(line: &str) -> Result<(char, i32), String> {
    if line.len() < 2 {
        return Err(format!("Line too short: '{}'", line));
    }
    // ... rest of function
}
```

**When to use each**:
- `continue`: Skip iteration in loop
- `return`: Exit function early

---

### 13. Error Output: `eprintln!` vs `println!`

#### `println!`

Prints to **standard output** (stdout):
- Normal program output
- Can be redirected: `program > file.txt`

#### `eprintln!`

Prints to **standard error** (stderr):
- Error messages
- Warnings
- Debug information
- Not redirected by default: `program > file.txt` still shows errors

#### Example from Our Code:

```rust
eprintln!("Error: File '{}' not found", filename);  // Error → stderr
println!("Password: {}", password);                  // Output → stdout
```

**Best Practice**:
- Use `eprintln!` for errors/warnings
- Use `println!` for normal output
- Allows proper output redirection

---

### 14. Process Exit: `std::process::exit()`

#### What it does:

Terminates the program immediately with an exit code.

#### Exit Codes:

- `0`: Success
- Non-zero: Error (convention: `1` for general error)

#### Example from Our Code:

```rust
fn handle_file_error(e: Error, filename: &str) -> ! {
    // ... print error ...
    std::process::exit(1);  // Exit with error code 1
}
```

**Why use it?**
- Signals failure to calling process/shell
- Allows scripts to detect errors
- Standard Unix convention

---

### 15. Function Return Types

#### Explicit Return Types

Rust functions must declare return type (except `main`):

```rust
fn function_name(param: Type) -> ReturnType {
    // ...
}
```

#### Examples from Our Code:

```rust
fn read_input_file(filename: &str) -> Result<String, Error>
// Returns: Result containing String or Error

fn parse_rotation(line: &str) -> Result<(char, i32), String>
// Returns: Result containing tuple (char, i32) or error String

fn apply_rotation(position: i32, direction: char, distance: i32) -> i32
// Returns: i32 (new position)

fn solve_puzzle(input: &str) -> u32
// Returns: u32 (count)

fn handle_file_error(e: Error, filename: &str) -> !
// Returns: ! (never returns)
```

**Why explicit types?**
- Type safety: Compiler catches mismatches
- Documentation: Clear what function returns
- No ambiguity: No guessing return types

---

## Code Walkthrough

### Step-by-Step Execution

```rust
fn main() {
    let filename = "input.txt";
    
    // 1. Read file - returns Result<String, Error>
    let input = match read_input_file(filename) {
        Ok(input) => input,        // Success: use the string
        Err(e) => {                // Error: handle it
            handle_file_error(e, filename);  // Prints error and exits
        }
    };
    
    // 2. Solve puzzle - input is guaranteed to be valid here
    let password = solve_puzzle(&input);  // Pass reference (borrow)
    
    // 3. Print result
    println!("Password: {}", password);
}
```

**Inside `solve_puzzle`**:

```rust
fn solve_puzzle(input: &str) -> u32 {
    let mut position = 50;  // Start position
    let mut count = 0;      // Counter for zeros
    
    // Iterate over each line
    for line in input.lines() {
        let line = line.trim();  // Remove whitespace
        
        // Skip empty lines
        if line.is_empty() {
            continue;
        }
        
        // Parse and handle errors
        match parse_rotation(line) {
            Ok((direction, distance)) => {
                // Success: apply rotation
                position = apply_rotation(position, direction, distance);
                
                // Check if at zero
                if position == 0 {
                    count += 1;  // Increment counter
                }
            },
            Err(e) => {
                // Error: log and continue
                eprintln!("Warning: Invalid rotation '{}': {}", line, e);
                continue;
            }
        }
    }
    
    count  // Return final count (implicit return)
}
```

**Inside `parse_rotation`**:

```rust
fn parse_rotation(line: &str) -> Result<(char, i32), String> {
    // 1. Validate length
    if line.len() < 2 {
        return Err(format!("Line too short: '{}'", line));
    }
    
    // 2. Extract and validate direction
    let direction = match line.chars().next() {
        Some(d @ 'L') | Some(d @ 'R') => d,  // Valid: L or R
        _ => return Err(format!("Invalid direction in '{}'", line)),
    };
    
    // 3. Parse distance
    let distance = match line[1..].parse::<i32>() {
        Ok(d) => d,  // Success
        Err(e) => return Err(format!("Invalid number in '{}': {}", line, e)),
    };
    
    // 4. Return success
    Ok((direction, distance))
}
```

**Inside `apply_rotation`**:

```rust
fn apply_rotation(position: i32, direction: char, distance: i32) -> i32 {
    match direction {
        'R' => (position + distance) % 100,           // Right: add
        'L' => (position - distance + 100) % 100,    // Left: subtract (with wrap)
        _ => position,  // Unknown direction: no change
    }
}
```

---

## Rust Best Practices Demonstrated

### 1. **Explicit Error Handling**

✅ **Good** (our code):
```rust
match read_input_file(filename) {
    Ok(input) => input,
    Err(e) => handle_file_error(e, filename),
}
```

❌ **Bad**:
```rust
let input = read_to_string(filename).unwrap();  // Panics on error!
```

### 2. **Function Separation**

✅ **Good** (our code):
- Each function has single responsibility
- Easy to test
- Easy to modify

❌ **Bad**:
- One giant function doing everything
- Hard to test
- Hard to maintain

### 3. **Proper String Handling**

✅ **Good** (our code):
```rust
fn parse_rotation(line: &str) -> Result<(char, i32), String>
// Uses &str for input (borrowed)
```

❌ **Bad**:
```rust
fn parse_rotation(line: String) -> Result<(char, i32), String>
// Unnecessary ownership transfer
```

### 4. **Error Messages**

✅ **Good** (our code):
```rust
Err(format!("Invalid number in '{}': {}", line, e))
// Specific, includes context
```

❌ **Bad**:
```rust
Err("Parse error".to_string())
// Vague, no context
```

### 5. **Type Safety**

✅ **Good** (our code):
- Explicit types everywhere
- Compiler catches errors
- No runtime surprises

---

## Key Takeaways

### Rust Philosophy

1. **Safety First**: Compiler prevents many bugs at compile time
2. **Explicit over Implicit**: Must handle errors, no hidden failures
3. **Zero-cost Abstractions**: High-level code, low-level performance
4. **Ownership System**: Prevents memory bugs without garbage collection

### What Makes This Code Appropriate 🦀

1. ✅ **No panics**: All errors handled explicitly
2. ✅ **Clear error messages**: Users know what went wrong
3. ✅ **Modular design**: Easy to test and maintain
4. ✅ **Type safety**: Compiler ensures correctness
5. ✅ **Proper I/O**: Errors go to stderr, output to stdout
6. ✅ **Exit codes**: Signals success/failure correctly
