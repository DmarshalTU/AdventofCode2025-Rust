# Advent of Code - Rust Solutions

This repository contains my solutions to [Advent of Code](https://adventofcode.com/) puzzles, implemented in Rust with a focus on learning proper Rust practices and production-ready code patterns.

## 🦀 Approach

Each solution emphasizes:
- **Explicit error handling** with `Result<T, E>`
- **Clean code organization** with separated functions
- **Production-ready patterns** (no panics, proper error messages)
- **Type safety** and compiler-enforced correctness
- **Detailed explanations** of Rust concepts and syntax

## 📚 Daily Recaps

Each day includes a comprehensive `RECAP.md` file that explains:
- Problem overview and solution approach
- Rust concepts used (with detailed explanations)
- Code walkthrough and best practices
- Key takeaways and learning points

### Solutions

- **[Day 1: Secret Entrance](./day_1/RECAP.md)** ✅
  - Part 1: [RECAP.md](./day_1/RECAP.md) - Count zeros at end of rotation
  - Part 2: [RECAP2.md](./day_1/RECAP2.md) - Count zeros during rotation
  - File I/O with error handling, tuple return types, step-by-step simulation
- **[Day 2: TBD](./day_2/)** 🚧
  - Coming soon...

## 🛠️ Running Solutions

Each day is a separate Cargo project. To run a solution:

```bash
cd day_1
cargo run
```

## 📖 Learning Resources

This repository serves as both:
1. **Solutions** to Advent of Code puzzles
2. **Learning material** for Rust concepts and best practices

Each recap document provides detailed explanations suitable for Rust learners.

## 📝 Structure

```
aoc/
├── day_1/
│   ├── src/
│   │   └── main.rs      # Solution code (Part 1 & 2)
│   ├── input.txt        # Puzzle input
│   ├── RECAP.md         # Part 1 detailed explanation
│   ├── RECAP2.md        # Part 2 detailed explanation
│   └── Cargo.toml       # Project configuration
├── day_2/
│   └── ...
└── README.md            # This file
```

## 🎯 Goals

- Learn Rust through practical problem-solving
- Build production-ready code from day one
- Document concepts for future reference
- Share knowledge with the Rust community

---

**Note**: This repository focuses on learning and best practices. Solutions prioritize clarity and educational value over optimization.

