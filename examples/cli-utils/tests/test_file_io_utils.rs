use cli_utils::file_io_utils::*;
use std::fs;
use std::path::PathBuf;
use std::env;

fn create_temp_file_for_test(content: &str) -> PathBuf {
    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join(format!("integration_test_file_{}.txt", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()));
    write_string_to_file(&file_path, content).unwrap();
    file_path
}

fn cleanup_test_file(file_path: &PathBuf) {
    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_read_write_integration() {
    let test_content = "Hello, World!\nThis is a test file for integration testing.";
    let file_path = create_temp_file_for_test(test_content);
    
    // Test reading
    let read_content = read_file_to_string(&file_path).unwrap();
    assert_eq!(read_content, test_content);
    
    // Test overwriting
    let new_content = "New content for testing";
    write_string_to_file(&file_path, new_content).unwrap();
    let updated_content = read_file_to_string(&file_path).unwrap();
    assert_eq!(updated_content, new_content);
    
    cleanup_test_file(&file_path);
}

#[test]
fn test_file_append_integration() {
    let initial_content = "Initial line";
    let file_path = create_temp_file_for_test(initial_content);
    
    // Test appending
    let append_content = "\nAppended line";
    append_to_file(&file_path, append_content).unwrap();
    
    let final_content = read_file_to_string(&file_path).unwrap();
    assert_eq!(final_content, format!("{}{}", initial_content, append_content));
    
    cleanup_test_file(&file_path);
}

#[test]
fn test_file_lines_integration() {
    let test_lines = "Line 1\nLine 2\nLine 3\nLine 4";
    let file_path = create_temp_file_for_test(test_lines);
    
    // Test reading lines
    let lines = read_lines(&file_path).unwrap();
    assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3", "Line 4"]);
    
    // Test reading first n lines
    let first_lines = read_first_n_lines(&file_path, 2).unwrap();
    assert_eq!(first_lines, vec!["Line 1", "Line 2"]);
    
    cleanup_test_file(&file_path);
}

#[test]
fn test_file_operations_integration() {
    let test_content = "Test content for file operations";
    let file_path = create_temp_file_for_test(test_content);
    
    // Test file exists
    assert!(file_exists(&file_path));
    
    // Test file size
    let size = file_size(&file_path).unwrap();
    assert_eq!(size, test_content.len() as u64);
    
    // Test copy file
    let temp_dir = env::temp_dir();
    let copy_path = temp_dir.join(format!("copy_test_{}.txt", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()));
    
    let bytes_copied = copy_file(&file_path, &copy_path).unwrap();
    assert_eq!(bytes_copied, test_content.len() as u64);
    assert!(file_exists(&copy_path));
    
    // Test delete file
    delete_file(&file_path).unwrap();
    assert!(!file_exists(&file_path));
    
    cleanup_test_file(&copy_path);
}

#[test]
fn test_write_lines_integration() {
    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join(format!("lines_test_{}.txt", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()));
    
    let test_lines = vec![
        "First line".to_string(),
        "Second line".to_string(),
        "Third line".to_string(),
    ];
    
    write_lines(&file_path, &test_lines).unwrap();
    
    let content = read_file_to_string(&file_path).unwrap();
    assert_eq!(content, "First line\nSecond line\nThird line");
    
    let read_lines_result = read_lines(&file_path).unwrap();
    assert_eq!(read_lines_result, test_lines);
    
    cleanup_test_file(&file_path);
}

#[test]
fn test_directory_operations_integration() {
    let temp_dir = env::temp_dir();
    let test_dir = temp_dir.join(format!("test_dir_{}", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()));
    
    // Test creating directory
    create_dir_all(&test_dir).unwrap();
    assert!(test_dir.exists());
    assert!(test_dir.is_dir());
    
    // Clean up
    let _ = fs::remove_dir(&test_dir);
}