pub mod events_v1 {
    tonic::include_proto!("events_v1");

    // Include Google proto types manually
    pub mod google {
        pub mod r#type {
            include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));
        }
    }
}

// Add a re-export at crate root to satisfy `super::google`
pub use events_v1::google;
