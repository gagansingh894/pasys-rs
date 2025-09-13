pub mod ledger_v1 {
    tonic::include_proto!("ledger_v1");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("ledger_v1_descriptor");

    // Include Google proto types manually
    pub mod google {
        pub mod r#type {
            include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
        }
    }
}

// Add a re-export at crate root to satisfy `super::google`
pub use ledger_v1::google;
