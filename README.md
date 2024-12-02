# File Organizer

This Rust program organizes files in the current directory by moving them into a specified subdirectory based on their file extension. The user specifies the subdirectory name and the desired file extension through the command-line.

## Features

- Create a subdirectory (if it doesn’t already exist).
- Filter and move files based on a specific file extension.
- Prints status messages to track the script’s operations.

## Usage

### How to Run

```bash
cargo run subdirectory_name file_extension
```

### Arguments

1. subdirectory_name: The name of the subdirectory to create or use.
2. file_extension: The file extension to filter files for moving.

### Example

```bash
cargo run txt_files txt
```

This will:

1. Create a subdirectory named txt_files (if it doesn't already exist).
2. Move all .txt files from the current directory into txt_files.
