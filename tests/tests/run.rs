mod cases;

use dropset_tests::{run_and_report, setup};

#[test]
fn cache_test() {
    panic!("intentional failure to verify CI caches on failure");
}

#[test]
fn all_cases() {
    let setup = setup();
    run_and_report("Entrypoint", cases::entrypoint::Case::ALL, &setup);
    run_and_report("RegisterMarket", cases::register_market::Case::ALL, &setup);
}
