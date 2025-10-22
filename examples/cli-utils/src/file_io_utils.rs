//! File input/output utility functions
//! 
//! This module provides file I/O utilities including:
//! - Reading file contents
//! - Writing to files
//! - Appending to files
//! - File system operations

use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

/// Reads the entire contents of a file and returns it as a String
/// 
/// # Arguments
/// * `file_path` - The path to the file to read
/// 
/// # Returns
/// The file contents as a String or an error
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::read_file_to_string;
/// // let contents = read_file_to_string("example.txt").unwrap();
/// ```
pub fn read_file_to_string<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    std::fs::read_to_string(file_path)
}

/// Writes a string to a file, creating the file if it doesn't exist or overwriting if it does
/// 
/// # Arguments
/// * `file_path` - The path to the file to write to
/// * `content` - The content to write to the file
/// 
/// # Returns
/// Result indicating success or failure
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::write_string_to_file;
/// // write_string_to_file("example.txt", "Hello, World!").unwrap();
/// ```
pub fn write_string_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    std::fs::write(file_path, content)
}

/// Appends a string to an existing file, creating the file if it doesn't exist
/// 
/// # Arguments
/// * `file_path` - The path to the file to append to
/// * `content` - The content to append to the file
/// 
/// # Returns
/// Result indicating success or failure
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::append_to_file;
/// // append_to_file("example.txt", "\nNew line").unwrap();
/// ```
pub fn append_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    file.write_all(content.as_bytes())
}

/// Reads a file line by line and returns a vector of lines
/// 
/// # Arguments
/// * `file_path` - The path to the file to read
/// 
/// # Returns
/// A vector of lines from the file or an error
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::read_lines;
/// // let lines = read_lines("example.txt").unwrap();
/// ```
pub fn read_lines<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/// Writes a vector of lines to a file
/// 
/// # Arguments
/// * `file_path` - The path to the file to write to
/// * `lines` - The lines to write to the file
/// 
/// # Returns
/// Result indicating success or failure
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::write_lines;
/// // let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
/// // write_lines("example.txt", &lines).unwrap();
/// ```
pub fn write_lines<P: AsRef<Path>>(file_path: P, lines: &[String]) -> io::Result<()> {
    let content = lines.join("\n");
    write_string_to_file(file_path, &content)
}

/// Checks if a file exists
/// 
/// # Arguments
/// * `file_path` - The path to check
/// 
/// # Returns
/// `true` if the file exists, `false` otherwise
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::file_exists;
/// // let exists = file_exists("example.txt");
/// ```
pub fn file_exists<P: AsRef<Path>>(file_path: P) -> bool {
    file_path.as_ref().exists() && file_path.as_ref().is_file()
}

/// Gets the size of a file in bytes
/// 
/// # Arguments
/// * `file_path` - The path to the file
/// 
/// # Returns
/// The size of the file in bytes or an error
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::file_size;
/// // let size = file_size("example.txt").unwrap();
/// ```
pub fn file_size<P: AsRef<Path>>(file_path: P) -> io::Result<u64> {
    let metadata = std::fs::metadata(file_path)?;
    Ok(metadata.len())
}

/// Creates a directory and all parent directories if they don't exist
/// 
/// # Arguments
/// * `dir_path` - The path to the directory to create
/// 
/// # Returns
/// Result indicating success or failure
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::create_dir_all;
/// // create_dir_all("path/to/directory").unwrap();
/// ```
pub fn create_dir_all<P: AsRef<Path>>(dir_path: P) -> io::Result<()> {
    std::fs::create_dir_all(dir_path)
}

/// Copies a file from source to destination
/// 
/// # Arguments
/// * `source` - The source file path
/// * `destination` - The destination file path
/// 
/// # Returns
/// The number of bytes copied or an error
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::copy_file;
/// // let bytes_copied = copy_file("source.txt", "destination.txt").unwrap();
/// ```
pub fn copy_file<P: AsRef<Path>>(source: P, destination: P) -> io::Result<u64> {
    std::fs::copy(source, destination)
}

/// Deletes a file
/// 
/// # Arguments
/// * `file_path` - The path to the file to delete
/// 
/// # Returns
/// Result indicating success or failure
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::delete_file;
/// // delete_file("unwanted.txt").unwrap();
/// ```
pub fn delete_file<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    std::fs::remove_file(file_path)
}

/// Reads the first n lines from a file
/// 
/// # Arguments
/// * `file_path` - The path to the file to read
/// * `n` - The number of lines to read
/// 
/// # Returns
/// A vector containing the first n lines or an error
/// 
/// # Examples
/// ```
/// use cli_utils::file_io_utils::read_first_n_lines;
/// // let lines = read_first_n_lines("example.txt", 5).unwrap();
/// ```
pub fn read_first_n_lines<P: AsRef<Path>>(file_path: P, n: usize) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn create_temp_file(content: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("test_file_{}.txt", rand::random::<u64>()));
        write_string_to_file(&file_path, content).unwrap();
        file_path
    }

    fn cleanup_temp_file(file_path: &Path) {
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_read_write_file() {
        let content = "Hello, World!\nThis is a test file.";
        let file_path = create_temp_file(content);
        
        let read_content = read_file_to_string(&file_path).unwrap();
        assert_eq!(read_content, content);
        
        cleanup_temp_file(&file_path);
    }

    #[test]
    fn test_append_to_file() {
        let initial_content = "Initial content";
        let append_content = "\nAppended content";
        let file_path = create_temp_file(initial_content);
        
        append_to_file(&file_path, append_content).unwrap();
        
        let final_content = read_file_to_string(&file_path).unwrap();
        assert_eq!(final_content, format!("{}{}", initial_content, append_content));
        
        cleanup_temp_file(&file_path);
    }

    #[test]
    fn test_read_lines() {
        let content = "Line 1\nLine 2\nLine 3";
        let file_path = create_temp_file(content);
        
        let lines = read_lines(&file_path).unwrap();
        assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
        
        cleanup_temp_file(&file_path);
    }

    #[test]
    fn test_write_lines() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("test_lines_{}.txt", rand::random::<u64>()));
        
        let lines = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        write_lines(&file_path, &lines).unwrap();
        
        let content = read_file_to_string(&file_path).unwrap();
        assert_eq!(content, "Line 1\nLine 2\nLine 3");
        
        cleanup_temp_file(&file_path);
    }

    #[test]
    fn test_file_exists() {
        let file_path = create_temp_file("test content");
        assert!(file_exists(&file_path));
        
        cleanup_temp_file(&file_path);
        assert!(!file_exists(&file_path));
    }

    #[test]
    fn test_file_size() {
        let content = "Hello, World!";
        let file_path = create_temp_file(content);
        
        let size = file_size(&file_path).unwrap();
        assert_eq!(size, content.len() as u64);
        
        cleanup_temp_file(&file_path);
    }

    #[test]
    fn test_read_first_n_lines() {
        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let file_path = create_temp_file(content);
        
        let lines = read_first_n_lines(&file_path, 3).unwrap();
        assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
        
        cleanup_temp_file(&file_path);
    }
}

// We need to add rand as a dependency for testing, but for now we'll use a simple alternative
mod rand {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub fn random<T>() -> T 
    where
        T: From<u64>,
    {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        T::from(now)
    }
}