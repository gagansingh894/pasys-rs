use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Root of proto root
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
        .join("proto");

    // Paths to proto files
    let event_proto_account_file = proto_root.join("pasys/events/v1/account.proto");
    let event_proto_refund_file = proto_root.join("pasys/events/v1/refund.proto");
    let event_proto_reconciliation_file = proto_root.join("pasys/events/v1/reconciliation.proto");
    let event_proto_fraud_file = proto_root.join("pasys/events/v1/fraud.proto");
    let event_proto_settlement_file = proto_root.join("pasys/events/v1/settlement.proto");
    let event_proto_transaction_file = proto_root.join("pasys/events/v1/transaction.proto");

    tonic_build::configure().out_dir(&out_dir).compile(
        &[
            event_proto_account_file.clone(),
            event_proto_refund_file.clone(),
            event_proto_reconciliation_file.clone(),
            event_proto_fraud_file.clone(),
            event_proto_settlement_file.clone(),
            event_proto_transaction_file.clone(),
        ],
        &[proto_root.to_str().unwrap()],
    )?;

    let protos = vec![
        event_proto_transaction_file,
        event_proto_account_file,
        event_proto_fraud_file,
        event_proto_reconciliation_file,
        event_proto_refund_file,
        event_proto_settlement_file,
    ];

    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }
    println!("cargo:rerun-if-changed={}", proto_root.display());

    Ok(())
}
