# Atomic

Atomic is an open source social network backend/api written entirely in Rust.

### Tech

Atomic uses these crates in order to function:

* [Rocket](https://rocket.rs/) - A simple, fast, and secure web framework.
* [Diesel](https://diesel.rs/) - A clean and simple ORM.
* [Rust-Argon2](https://crates.io/crates/rust-argon2) - An Argon2 implementation in Rust for securely hashing passwords.
* [Chrono](https://crates.io/crates/chrono) - A date and time crate for rust.
* [Serde](https://crates.io/crates/serde) - A JSON Serialization/Deserialization framework.
* [Rand](https://crates.io/crates/rand) - A simple Random Number Generaation crate.


### Installation
Atomic requires Rust Nightly and a PostgreSQL Database in order to function.
```sh
rustup toolchain install nightly
```
Configure your database in your Rocket.toml and .env files.
```toml
[global.databases]
atomic_db = {url = "postgres://username:password@host/database"}
```
```.env
DATABASE_URL=postgres://username:password@host/database
```
Install and run DieselCLI to configure database.
```
cargo install diesel_cli --no-default-features --features postgres
```
```
diesel setup
```
```
diesel run
```
And thats it! Now you can run and build using Cargo Build or Cargo Run respectively.


### Todos

 - Finish Auth System
 - Add Post Creation
 - Private Messaging(?)

License
----

Atomic is currently licensed under the GPLv3 License.
