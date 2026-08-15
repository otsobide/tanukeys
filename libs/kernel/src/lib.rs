//! # Kernel
//!
//! Core bounded context of Tanukeys. It temporarily hosts every module of
//! the platform until the domain grows enough to justify splitting them
//! into dedicated bounded contexts.
//!
//! ## Modules
//!
//! - [`users`]: basic user identity (platform id, user name, description).
//! - [`crypto_keys`]: cryptographic keys owned by users (protocol,
//!   algorithm, payload, timestamps).

pub mod crypto_keys;
pub mod users;
