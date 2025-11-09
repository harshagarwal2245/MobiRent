//! # Mobirent Library
//!
//! **Mobirent** is the core library for the car rental and remote fleet access system.
//! It implements the Proxy design pattern to provide secure and efficient access
//! to vehicle data, telemetry, and cache handling.
//!
//! ## Overview
//!
//! - **Proxy Pattern** for controlled access to remote fleet services  
//! - **Asynchronous** APIs using `async`/`await` and `tokio`  
//! - **Caching Layer** to reduce redundant network calls  
//! - **Telemetry Logging** for fleet operations  
//! - **Error Handling** with `thiserror`  
//! - Designed for **extensibility** and **testability**
//!
//! ## Example
//!
//! ```rust,ignore
//! use mobirent_lib::{FleetProxy, RemoteFleetAccess, CacheManager};
//!
//! #[tokio::main]
//! async fn main() {
//!     let remote = RemoteFleetAccess;
//!     let cache = CacheManager::default();
//!     let proxy = FleetProxy::new(remote, cache);
//!
//!     match proxy.get_fleet().await {
//!         Ok(fleet) => println!("Fetched {} cars", fleet.cars.len()),
//!         Err(e) => eprintln!("Error: {:?}", e),
//!     }
//! }
//! ```

// Internal modules — not exposed directly
mod model;
mod proxy;
mod remote;
mod cache;
mod telemetry;
mod errors;
mod traits;

// Public re-exports — defines the library’s external API surface

/// Core proxy interface providing cached and remote access to fleet data.
pub use proxy::FleetProxy;

/// Data models representing car and fleet information.
pub use model::{Car, Fleet};

/// Fleet telemetry logging utilities.
pub use telemetry::TelemetryLogger;

/// Cache management utilities for in-memory or file-backed caching.
pub use cache::CacheManager;

/// Error types used across the Mobirent library.
pub use errors::FleetError;

/// Trait definitions for Fleet access abstractions.
pub use traits::FleetAccess;

/// Remote implementation simulating cloud-based fleet data access.
pub use remote::RemoteFleetAccess;
