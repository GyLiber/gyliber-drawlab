use std::process::Command;
fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_drawlab"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn user_workflow_generates_and_checks_export() {
    let generated = run(&["generate", "--profile", "lab-5of36", "--boards", "2"]);
    assert!(generated.status.success());
    let record = std::str::from_utf8(&generated.stdout).unwrap();
    let checked = drawlab_core::verify(record).unwrap();
    assert_eq!(checked.boards.len(), 2);
    let path = std::env::temp_dir().join(format!("drawlab-test-{}.json", std::process::id()));
    std::fs::write(&path, record).unwrap();
    let result = run(&["verify", path.to_str().unwrap()]);
    std::fs::remove_file(&path).unwrap();
    assert!(result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stdout)
            .contains("Authenticity and randomness are not established")
    );
}
#[test]
fn errors_produce_no_partial_success_output() {
    for args in [
        vec!["generate", "--profile", "ithuba-lotto-2025"],
        vec!["generate", "--profile", "lab-5of36", "--boards", "101"],
        vec!["odds", "--profile", "missing"],
    ] {
        let r = run(&args);
        assert!(!r.status.success());
        assert!(r.stdout.is_empty());
        assert!(!r.stderr.is_empty());
    }
    let r = run(&["odds", "--profile", "lab-5of50-20"]);
    assert!(r.status.success());
    assert!(String::from_utf8_lossy(&r.stdout).contains("42375200"));
}
