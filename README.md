# zero2prod

A production-ready web service written in Rust, focusing on best practices for backend engineering, database management, and asynchronous runtime configurations.

---

## 🛠️ Tech Stack

* **Language:** Rust
* **Async Runtime:** `tokio`
* **HTTP Framework:** `actix-web`
* **Database:** PostgreSQL
* **Database Tooling:** `sqlx` (with `sqlx-cli`)
* **Containerization:** Docker

---

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed on your machine:

* Rust
* Docker
* sqlx-cli

To install `sqlx-cli` with PostgreSQL support, run:

cargo install --version="~0.8" sqlx-cli \
  --no-default-features \
  --features rustls,postgres

---

## ⚙️ Local Setup

### 1. Initialize the Database

Use the helper script to spin up the PostgreSQL Docker container, configure the application user, and run initial migrations:

chmod +x scripts/init_db.sh
./scripts/init_db.sh

Note: If you already have a Postgres container running, you can bypass container creation and run migrations against it with:
SKIP_DOCKER=true ./scripts/init_db.sh

### 2. Environment Configuration

The application expects database credentials via `DATABASE_URL`:

export DATABASE_URL=postgres://app:secret@localhost:5432/newsletter

---

## 🏃 Running the Application

To start the HTTP server locally:

cargo run

### Running Tests

Execute the test suite using `cargo`:

cargo test
