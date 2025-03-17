use std::env;

use quadit::file_manager::FileManager;

#[test]
fn test_set_podman_unit_location() {
    env::set_var("PODMAN_UNIT_PATH", "iamset");
    assert_eq!(
        FileManager::podman_unit_path(),
        "iamset",
        "Unexpected podman unit"
    );
}
