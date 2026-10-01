use filezen::organize_directory;
use std::fs;

#[test]
fn test_integration_organize_flow() {
    let test_dir = std::env::temp_dir().join("filezen_integration_test");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    // Create loose files
    fs::write(test_dir.join("photo.png"), b"image").unwrap();
    fs::write(test_dir.join("report.pdf"), b"doc").unwrap();
    fs::write(test_dir.join("video.mp4"), b"video").unwrap();
    fs::write(test_dir.join("bundle.tar.gz"), b"archive").unwrap();
    fs::write(test_dir.join("app.rs"), b"code").unwrap();
    fs::write(test_dir.join("unknown.dat"), b"misc").unwrap();

    // Create a subfolder with files that must NOT be touched
    let subfolder = test_dir.join("subfolder");
    fs::create_dir_all(&subfolder).unwrap();
    fs::write(subfolder.join("leave_me_alone.txt"), b"untouched").unwrap();

    // Run organization
    let summary = organize_directory(&test_dir).unwrap();
    assert_eq!(summary.moved_count, 6);
    assert_eq!(summary.warning_count, 0);

    // Verify categorized files
    assert!(test_dir.join("Images").join("photo.png").exists());
    assert!(test_dir.join("Documents").join("report.pdf").exists());
    assert!(test_dir.join("Videos").join("video.mp4").exists());
    assert!(test_dir.join("Archives").join("bundle.tar.gz").exists());
    assert!(test_dir.join("Code").join("app.rs").exists());
    assert!(test_dir.join("Misc").join("unknown.dat").exists());

    // Verify subfolder wasn't altered
    assert!(subfolder.join("leave_me_alone.txt").exists());

    // Clean up
    let _ = fs::remove_dir_all(&test_dir);
}
