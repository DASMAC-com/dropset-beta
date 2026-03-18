use dropset_tests::cases::entrypoint::EntrypointCase;
use dropset_tests::{run_and_report, setup};

#[test]
fn entrypoint_cases() {
    let setup = setup();
    run_and_report("Entrypoint", EntrypointCase::ALL, &setup);
}
