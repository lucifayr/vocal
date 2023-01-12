pub fn get_filename_from_path(path: &str) -> Option<&str> {
    path.split("/").last()?.split(".").next()
}

#[test]
fn test_get_filename_from_path() {
    assert_eq!(get_filename_from_path(""), Some(""));
    assert_eq!(get_filename_from_path("file"), Some("file"));
    assert_eq!(get_filename_from_path("file.mp3"), Some("file"));
    assert_eq!(get_filename_from_path("file.mp3.exe"), Some("file"));
    assert_eq!(get_filename_from_path("folder/file"), Some("file"));
    assert_eq!(get_filename_from_path("folder/file.mp3"), Some("file"));
    assert_eq!(
        get_filename_from_path("folder/folder/file.mp3"),
        Some("file")
    );
}
