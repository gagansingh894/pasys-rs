# PaSys - Payment and Settlement System

**PaSys** is a Rust-based, end-to-end **payment and settlement system** with a **double-entry ledger**, synchronous balance updates, and asynchronous event-driven reconciliation workflows.

---

## Project Structure

- `ledger/` – Rust library with core ledger logic (accounts, transactions, balances, idempotency, validation)
- `transactions-api/` – Rust library for HTTP/gRPC API servers using `ledger` as a dependency
- `pasys/` – CLI application to start the system, interact with APIs, and run administrative tasks

---

## Features

- ACID double-entry ledger for correctness
- Event-driven architecture for asynchronous settlement and fraud detection
- Idempotency, reconciliation, and refund support
- Modular Rust workspace for maintainable and testable code

---