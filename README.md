# LRU Cache in Rust

A generic **Least Recently Used (LRU) Cache** implemented from scratch in Rust, using a `HashMap` and a custom doubly linked list.

## How It Works

- A `HashMap` provides average O(1) key lookups.
- A doubly linked list tracks entries by usage, with the most recently used entry at the front.
- Accessing or updating an entry moves it to the front.
- When capacity is reached, the least recently used entry is evicted from the back.
- Uses `Rc`, `RefCell`, and `Weak` for shared node ownership and bidirectional links.

Cache operations (`get` and `put`) have average O(1) time complexity.

## Getting Started

Requires [Rust](https://www.rust-lang.org/tools/install) and Cargo.

```bash
git clone https://github.com/KunalNasa/LRU.git
cd lru-rs
cargo test
cargo build --release
```

## Usage

```rust
use lru_rs::LRUCache;

let mut cache = LRUCache::new(2);

cache.put("A".to_string(), 10);
cache.put("B".to_string(), 20);

assert_eq!(cache.get(&"A".to_string()), Some(10));

cache.put("C".to_string(), 30); // Evicts B
assert_eq!(cache.get(&"B".to_string()), None);
```

## Project Status

This is a learning project built to explore Rust ownership, borrowing, and data structures through implementing an LRU cache from scratch.

**This project is not intended for production use.** It is considered complete for its learning objectives, and there are currently no plans for further development or maintenance.
