mod domain;
pub mod repo;
pub mod service;

pub mod api;

pub const DEFAULT_WRITER_MAX_CONN: u32 = 4;
pub const DEFAULT_READER_MAX_CONN: u32 = 4;
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 5;
