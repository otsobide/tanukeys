//! # Kernel
//!
//! Core bounded context of Tanukeys. It temporarily hosts every module of
//! the platform until the domain grows enough to justify splitting them
//! into dedicated bounded contexts.
//!
//! ## Modules
//!
//! - [`users`]: basic user identity (platform id, user name, description).

pub mod users;
