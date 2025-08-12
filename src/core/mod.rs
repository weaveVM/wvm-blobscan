//! Core modules for the Blobscan Agent.
//!
//! This module contains all the core functionality organized into specialized
//! submodules for handling different aspects of the blob retrieval and storage
//! pipeline.
//!
//! ## Module Organization:
//! - `blobscan` - Blobscan API integration and blob fetching
//! - `constants` - Application-wide constants and configuration
//! - `env_var` - Environment variable management
//! - `eth` - Ethereum JSON-RPC client interactions
//! - `indexer` - Planetscale DB operations and metadata indexing
//! - `s3` - Load HyperBEAM S3 storage operations - ~s3@1.0 device
//! - `server_handlers` - HTTP API endpoint handlers
//! - `types` - Shared data structures and type definitions

pub mod blobscan;
pub mod constants;
pub mod env_var;
pub mod eth;
pub mod indexer;
pub mod s3;
pub mod server_handlers;
pub mod types;
