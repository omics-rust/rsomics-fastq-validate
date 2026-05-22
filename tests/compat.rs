use std::process::Command;

fn ours() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rsomics-fastq-validate"))
}

// No canonical FASTQ-validator CLI exists to diff against. Self-correctness: a
// well-formed FASTQ passes (exit 0); one with a quality/sequence length mismatch
// fails (non-zero).
#[test]
fn accepts_valid_rejects_invalid() {
    let dir = std::env::temp_dir().join("rsomics-fastq-validate-compat");
    std::fs::create_dir_all(&dir).unwrap();
    let ok = dir.join("ok.fq");
    std::fs::write(&ok, "@r1\nACGT\n+\nIIII\n@r2\nTTGG\n+\nFFFF\n").unwrap();
    let bad = dir.join("bad.fq");
    std::fs::write(&bad, "@r1\nACGT\n+\nII\n").unwrap();

    assert!(
        ours().arg(&ok).status().unwrap().success(),
        "valid FASTQ must pass"
    );
    assert!(
        !ours().arg(&bad).status().unwrap().success(),
        "qual/seq length mismatch must fail"
    );
}
