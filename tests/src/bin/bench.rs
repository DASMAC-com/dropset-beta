use dropset_tests::cases::entrypoint::EntrypointCase;
use dropset_tests::{run_and_report, setup};

fn main() {
    let setup = setup();
    run_and_report("Entrypoint", EntrypointCase::ALL, &setup);
    // Add additional case groups here as the test suite grows.
}
