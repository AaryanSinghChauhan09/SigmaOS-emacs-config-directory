// Integration test for SigmaOS-emacs-config-directory
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_emacs_config_directory as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
