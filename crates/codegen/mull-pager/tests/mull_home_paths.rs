//! `MULL_HOME` override tests in an isolated binary so `mull_home()`'s
//! process-wide `OnceLock` initializes from the overridden env var.

use std::path::PathBuf;

#[test]
fn mull_home_override_path_helpers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mull_home = tmp.path().to_path_buf();
    unsafe {
        std::env::set_var("MULL_HOME", &mull_home);
    }

    assert_eq!(
        mull_pager::util::pager_toml_path(),
        mull_home.join("pager.toml")
    );
    assert_eq!(mull_pager::util::display_mull_home_prefix(), "$MULL_HOME");
    assert_eq!(
        mull_pager::util::display_user_mull_path("config.toml"),
        "$MULL_HOME/config.toml"
    );

    let memory_path = mull_home.join("memory/MEMORY.md");
    assert_eq!(
        mull_pager::util::abbreviate_path(&memory_path.display().to_string()),
        "$MULL_HOME/memory/MEMORY.md"
    );

    assert!(mull_pager::util::is_under_user_mull_home(&memory_path));
    assert!(!mull_pager::util::is_under_user_mull_home(
        PathBuf::from("/tmp/other").as_path()
    ));
}
