# Concurrent Cache Engine

A generic, thread-safe cache engine developed in Rust, designed to explore memory management, data sharing and concurrent processing.

The project aims to implement a cache capable of handling simultaneous requests from multiple threads, reusing previously computed results and preventing redundant operations.

## Features

- Generic key-value storage.
- Configurable maximum capacity.
- FIFO (First In, First Out) eviction strategy.
- Duplicate key prevention.
- Reference-based value retrieval.
- Element insertion and removal.
- Cache clearing and size inspection.
- Thread-safe shared access.
- Concurrent request processing.
- Duplicate computation prevention.

## Technologies

- **Rust** — Core language.
- **Standard Library (`std`)** — Core functionality.
- **Arc** — Shared ownership across threads.
- **Mutex / RwLock** — Thread synchronization.
- **HashMap / Vec** — Data storage structures.

No external dependencies are required.

## Project Architecture

```text
concurrent-cache/
├── src/
│   ├── main.rs
│   ├── cache.rs
│   ├── user.rs
│   ├── worker.rs
│   └── metrics.rs
└── Cargo.toml
```

## Core Concepts

This project explores several Rust concepts through practical implementation:

- **Ownership & Borrowing:** Managing data ownership and references without unnecessary copies.
- **Generics & Traits:** Building reusable data structures with type constraints.
- **Memory Safety:** Understanding references, lifetimes and safe memory access.
- **Concurrency:** Sharing data across threads while preventing race conditions.
- **Synchronization:** Coordinating simultaneous read and write operations.
- **Thread Safety:** Exploring `Send`, `Sync`, `Arc`, `Mutex` and `RwLock`.
- **Performance:** Evaluating caching strategies and reducing redundant computations.

## Expected Behavior

The cache will be shared across multiple worker threads.

When multiple threads request the same resource, the system should:

1. Check whether the requested value already exists in the cache.
2. Return the cached value when available.
3. Execute the expensive operation only once when the value is missing.
4. Store the result and make it available to other threads.

This behavior aims to minimize redundant processing while maintaining data consistency.

## Learning Goals

The main goal is to understand how Rust handles shared ownership, synchronization and memory safety in concurrent applications.

The project also explores practical cache design decisions, synchronization strategies and performance trade-offs in multithreaded systems.

---

**Status:** In development

**Language:** Rust
