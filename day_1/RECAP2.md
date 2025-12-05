# Advent of Code Day 1 - Part 2: Rust Solution Recap

## Part 2 Problem Overview

**Challenge**: Count how many times a dial (0-99) points at 0 **during** each rotation (every click/step), not just at the end.

**Key Difference from Part 1**:
- Part 1: Count zeros only **after** each rotation completes
- Part 2: Count zeros **during** each rotation (every position the dial passes through)

**Example**:
```
Start: 50
L68 → 82; during rotation passes through 0 once ← count!
L30 → 52
R48 → 0; ends at 0 ← count!
L5 → 95
R60 → 55; during rotation passes through 0 once ← count!
L55 → 0; ends at 0 ← count!
L1 → 99
L99 → 0; ends at 0 ← count!
R14 → 14
L82 → 32; during rotation passes through 0 once ← count!

Result: 6 times at position 0
(3 at end of rotation + 3 during rotation)
```

**Important Edge Case**:
- Large rotations like `R1000` from position 50 will pass through 0 **ten times** (wrapping around multiple times)
- Must simulate each step, not just calculate the final position

---

## Solution Approach: Step-by-Step Simulation

### Part 1 vs Part 2

**Part 1 Approach** (Direct Calculation):
```rust
// Calculate final position directly
position = (position + distance) % 100;
if position == 0 {
    count += 1;  // Only check at the end
}
```

**Part 2 Approach** (Step-by-Step Simulation):
```rust
// Simulate each step of the rotation
for _ in 0..distance {
    current = (current + 1) % 100;  // Move one step
    if current == 0 {
        zero_count += 1;  // Check every step
    }
}
```

### Why Step-by-Step?

- **Large rotations**: `R1000` wraps around 10 times, passing through 0 multiple times
- **Intermediate positions**: Need to check every position during movement
- **Accurate counting**: Can't use direct calculation - must simulate

---

## New Rust Concepts for Part 2

### 1. Tuple Return Types

#### What is a Tuple?

A tuple is a fixed-size collection of values of different types:

```rust
let tuple: (i32, u32) = (42, 100);
//        ^^^^^^^^^^^^
//        Type annotation: (i32, u32)
```

#### Tuple Syntax

```rust
(value1, value2, value3)  // Create tuple
let (a, b) = tuple;       // Destructure (unpack)
tuple.0                   // Access first element
tuple.1                   // Access second element
```

#### Example from Our Code:

```rust
fn apply_rotation_with_zero_count(
    position: i32, 
    direction: char, 
    distance: i32
) -> (i32, u32) {  // Returns tuple: (new_position, zero_count)
    // ...
    (current, zero_count)  // Return both values
}
```

**Why use tuples?**
- Return multiple values from a function
- Group related data together
- Type-safe: compiler ensures correct types

---

### 2. Destructuring Tuples

#### What is Destructuring?

Extracting values from a tuple into separate variables.

#### Syntax:

```rust
let (variable1, variable2) = tuple;
```

#### Example from Our Code:

```rust
let (new_position, zeros_during_rotation) = 
    apply_rotation_with_zero_count(position, direction, distance);
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//  Destructure the returned tuple into two variables
```

**Breakdown**:
- Function returns `(i32, u32)`
- `let (new_position, zeros_during_rotation)` unpacks it
- `new_position` gets the `i32` (new position)
- `zeros_during_rotation` gets the `u32` (count of zeros)

**Alternative (without destructuring)**:
```rust
let result = apply_rotation_with_zero_count(position, direction, distance);
let new_position = result.0;           // Access by index
let zeros_during_rotation = result.1;   // Access by index
```

**Why destructuring is better**:
- More readable: clear variable names
- Less error-prone: can't mix up indices
- Idiomatic Rust: preferred style

---

### 3. Range Iteration: `0..distance`

#### What is a Range?

A range represents a sequence of numbers. Rust has two types:

```rust
0..distance      // Range (exclusive end): 0, 1, 2, ..., distance-1
0..=distance      // Inclusive range: 0, 1, 2, ..., distance
```

#### Example from Our Code:

```rust
for _ in 0..distance {
    // Execute 'distance' times
    // _ means we don't use the loop variable
}
```

**Breakdown**:
- `0..distance`: Creates range from 0 to distance-1
- `for _ in`: Iterate over range, ignore the value
- Executes the block `distance` times

**Why use `_`?**
- We don't need the loop counter value
- `_` tells Rust: "I'm intentionally ignoring this"
- Prevents unused variable warnings

**Alternative (if we needed the counter)**:
```rust
for step in 0..distance {
    println!("Step {}", step);  // Use the counter
}
```

---

### 4. Step-by-Step State Mutation

#### The Pattern

Instead of calculating the final state directly, we update state incrementally:

```rust
let mut current = position;  // Start state
let mut zero_count = 0;      // Accumulator

for _ in 0..distance {
    // Update state one step at a time
    current = (current + 1) % 100;
    
    // Check condition at each step
    if current == 0 {
        zero_count += 1;
    }
}
```

**Why this pattern?**
- **Intermediate checks**: Need to examine state at each step
- **Complex logic**: Can't derive final state directly
- **Accumulation**: Build up result over multiple steps

---

## Code Walkthrough: Part 2

### New Function: `apply_rotation_with_zero_count`

```rust
fn apply_rotation_with_zero_count(
    position: i32, 
    direction: char, 
    distance: i32
) -> (i32, u32) {
    let mut current = position;      // Start at current position
    let mut zero_count = 0;          // Initialize counter
    
    // Simulate each step of the rotation
    for _ in 0..distance {
        match direction {
            'R' => {
                // Move right: increment position
                current = (current + 1) % 100;
            },
            'L' => {
                // Move left: decrement position (with wrap)
                current = (current - 1 + 100) % 100;
            },
            _ => break,  // Unknown direction: exit loop
        }
        
        // Check if we're at position 0
        if current == 0 {
            zero_count += 1;  // Increment counter
        }
    }
    
    // Return both the new position and zero count
    (current, zero_count)
}
```

**Step-by-Step Execution Example** (`L68` from position 50):

```
Initial: current = 50, zero_count = 0

Step 1:  current = 49, zero_count = 0
Step 2:  current = 48, zero_count = 0
...
Step 50: current = 0,  zero_count = 1  ← Found zero!
Step 51: current = 99, zero_count = 1
...
Step 68: current = 82, zero_count = 1

Final: (82, 1)
```

### Updated `solve_puzzle` Function

```rust
fn solve_puzzle(input: &str) -> u32 {
    let mut position = 50;  // Start position
    let mut count = 0;      // Total zero count
    
    for line in input.lines() {
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }
        
        match parse_rotation(line) {
            Ok((direction, distance)) => {
                // Get both new position and zeros encountered
                let (new_position, zeros_during_rotation) = 
                    apply_rotation_with_zero_count(position, direction, distance);
                
                // Update state
                position = new_position;
                count += zeros_during_rotation;  // Add to total
            },
            Err(e) => {
                eprintln!("Warning: Invalid rotation '{}': {}", line, e);
                continue;
            }
        }
    }
    
    count  // Return total count
}
```

**Key Changes from Part 1**:
1. **Tuple destructuring**: `let (new_position, zeros_during_rotation) = ...`
2. **Accumulate zeros**: `count += zeros_during_rotation` (not just check final position)
3. **Step-by-step**: Function simulates each step, not just calculates final position

---

## Handling Large Rotations

### Example: `R1000` from position 50

**What happens**:
- Start at 50
- Move right 1000 steps
- Wraps around: 50 → 51 → ... → 99 → 0 → 1 → ... → 99 → 0 → ...
- Passes through 0 **ten times**:
  - Step 50: 50 → 0 (first time)
  - Step 150: wraps to 0 (second time)
  - Step 250: wraps to 0 (third time)
  - ... (continues every 100 steps)
  - Step 950: wraps to 0 (tenth time)

**Why direct calculation doesn't work**:
```rust
// Part 1 approach (wrong for Part 2):
position = (50 + 1000) % 100 = 50
if position == 0 { count += 1; }  // Only checks final position: 50 ≠ 0
// Misses all 10 zeros during rotation!
```

**Step-by-step approach (correct)**:
```rust
// Part 2 approach:
for _ in 0..1000 {
    current = (current + 1) % 100;
    if current == 0 { count += 1; }  // Checks every step
}
// Correctly counts all 10 zeros
```

---

## Rust Concepts Summary

### New Concepts in Part 2

1. **Tuple Return Types** `(T, U)`
   - Return multiple values from a function
   - Type-safe grouping of related data

2. **Tuple Destructuring** `let (a, b) = tuple`
   - Extract values from tuples
   - More readable than index access

3. **Range Iteration** `0..distance`
   - Create sequences of numbers
   - Use `_` to ignore loop variable

4. **Step-by-Step Simulation**
   - Update state incrementally
   - Check conditions at each step
   - Accumulate results over iterations

### Concepts Reused from Part 1

- `Result<T, E>` error handling
- Pattern matching with `match`
- String parsing and validation
- Modular arithmetic
- Mutability with `let mut`
- Control flow (`continue`, early returns)

---

## Key Takeaways

### When to Use Step-by-Step Simulation

Use step-by-step simulation when:
- ✅ Need to check intermediate states
- ✅ Can't calculate final state directly
- ✅ Need to accumulate values during iteration
- ✅ Logic depends on each step, not just the end result

### When to Use Direct Calculation

Use direct calculation when:
- ✅ Only care about final state
- ✅ Can derive result mathematically
- ✅ Performance matters (direct is faster)
- ✅ Logic only depends on start and end

### Part 2 Lessons

1. **Simulation vs Calculation**: Sometimes you must simulate, not calculate
2. **Multiple Return Values**: Tuples let you return related data together
3. **Destructuring**: Makes code more readable than index access
4. **Incremental Updates**: Some problems require step-by-step state updates

---

## Performance Considerations

### Part 1 (Direct Calculation)
- **Time Complexity**: O(1) per rotation
- **Space Complexity**: O(1)
- **Fast**: Single calculation per rotation

### Part 2 (Step-by-Step)
- **Time Complexity**: O(distance) per rotation
- **Space Complexity**: O(1)
- **Slower**: Must iterate through each step

**Trade-off**:
- Part 2 is slower but necessary for correctness
- For large distances (e.g., 1000), Part 2 does 1000 operations vs Part 1's 1 operation
- However, Part 2 is still fast enough for this problem size

---

## Conclusion

Part 2 demonstrates:
- ✅ **Tuple return types** for multiple values
- ✅ **Destructuring** for clean code
- ✅ **Step-by-step simulation** when direct calculation isn't sufficient
- ✅ **Range iteration** for controlled loops
- ✅ **Incremental state updates** with accumulation
