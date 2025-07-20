# Test Cases for `ls` Command (With `-l`, `-a`, `-F` Flags)

This document provides comprehensive test cases for verifying the functionality of an `ls` (list) command implementation. These tests cover the basic `ls` command as well as the `-l`, `-a`, and `-F` flags and their combinations.

## Prerequisites

- A working `ls` command implementation (referenced as `./ls` in examples)
- Unix-like shell environment (bash, zsh, etc.)
- Basic file system permissions for testing

## Test Environment Setup

Create a test directory with various file types for comprehensive testing:

```bash
mkdir ls_test_dir
cd ls_test_dir

# Create regular files
echo "content1" > file1.txt
echo "content2" > file2.log
touch empty_file

# Create hidden files
echo "hidden content" > .hidden_file
touch .hidden_empty

# Create directories
mkdir normal_dir
mkdir .hidden_dir

# Create executable file
echo '#!/bin/bash\necho "Hello"' > script.sh
chmod +x script.sh

# Create symlinks (if supported)
ln -s file1.txt link_to_file
ln -s normal_dir link_to_dir
ln -s nonexistent broken_link

# Create special files (if possible)
mkfifo named_pipe 2>/dev/null || echo "Named pipe creation skipped"
```

## Test Cases

### 1. Basic `ls` Command (No Flags)

**Test Command:**
```bash
./ls
```

**Expected Result:**
- Lists only visible files and directories
- Should NOT show hidden files (starting with `.`)
- Simple list format (names only)
- Should show: `empty_file`, `file1.txt`, `file2.log`, `link_to_dir`, `link_to_file`, `normal_dir`, `script.sh`

---

### 2. `ls -a` Flag (Show All Files)

**Test Command:**
```bash
./ls -a
```

**Expected Result:**
- Shows ALL files including hidden ones
- Should include `.` (current directory) and `..` (parent directory)
- Should show: `.`, `..`, `.hidden_dir`, `.hidden_empty`, `.hidden_file`, `broken_link`, `empty_file`, `file1.txt`, `file2.log`, `link_to_dir`, `link_to_file`, `named_pipe`, `normal_dir`, `script.sh`

---

### 3. `ls -l` Flag (Long Format)

**Test Command:**
```bash
./ls -l
```

**Expected Result:**
- Detailed listing with file permissions, links, owner, group, size, date/time
- Format: `permissions links owner group size date time name`
- Example output format:
```
-rw-r--r-- 1 user group    9 Jul 20 10:30 empty_file
-rw-r--r-- 1 user group    9 Jul 20 10:30 file1.txt
-rw-r--r-- 1 user group    9 Jul 20 10:30 file2.log
lrwxrwxrwx 1 user group    9 Jul 20 10:30 link_to_dir -> normal_dir
lrwxrwxrwx 1 user group    9 Jul 20 10:30 link_to_file -> file1.txt
drwxr-xr-x 2 user group 4096 Jul 20 10:30 normal_dir
-rwxr-xr-x 1 user group   24 Jul 20 10:30 script.sh
```

---

### 4. `ls -F` Flag (Classify Files)

**Test Command:**
```bash
./ls -F
```

**Expected Result:**
- Appends indicators to file names based on type:
  - `/` for directories
  - `*` for executable files
  - `@` for symbolic links
  - `|` for FIFOs (named pipes)
  - `=` for sockets
- Expected output: `empty_file`, `file1.txt`, `file2.log`, `link_to_dir@`, `link_to_file@`, `normal_dir/`, `script.sh*`

---

### 5. `ls -la` (Combine -l and -a)

**Test Command:**
```bash
./ls -la
```

**Expected Result:**
- Long format listing including hidden files
- Should show detailed info for all files including `.` and `..`
- Combines both `-l` and `-a` functionality

---

### 6. `ls -lF` (Combine -l and -F)

**Test Command:**
```bash
./ls -lF
```

**Expected Result:**
- Long format with file type indicators
- Detailed listing with classification symbols
- Example: `-rwxr-xr-x 1 user group 24 Jul 20 10:30 script.sh*`

---

### 7. `ls -aF` (Combine -a and -F)

**Test Command:**
```bash
./ls -aF
```

**Expected Result:**
- Shows all files (including hidden) with type indicators
- Should include `.hidden_dir/` and classification symbols for all files

---

### 8. `ls -laF` (All Three Flags Combined)

**Test Command:**
```bash
./ls -laF
```

**Expected Result:**
- Long format listing of all files with type indicators
- Most comprehensive output combining all three flags
- Shows hidden files, detailed info, and classification symbols

---

### 9. List Specific Directory

**Test Command:**
```bash
./ls normal_dir
./ls -l normal_dir
./ls -a normal_dir
./ls -F normal_dir
```

**Expected Result:**
- Lists contents of specified directory
- Should work with all flag combinations
- For empty directory, should show nothing (or just `.` and `..` with `-a`)

---

### 10. List Multiple Items

**Test Command:**
```bash
./ls file1.txt normal_dir
./ls -l file1.txt normal_dir
```

**Expected Result:**
- Shows specified file and directory contents
- Should handle mixed file/directory arguments
- May show directory name headers when listing multiple items

---

### 11. List Non-existent File

**Test Command:**
```bash
./ls nonexistent_file
```

**Expected Result:**
- Prints error: `ls: cannot access 'nonexistent_file': No such file or directory`
- Exit code should be non-zero

---

### 12. List Empty Directory

**Test Command:**
```bash
mkdir empty_test_dir
./ls empty_test_dir
./ls -a empty_test_dir
```

**Expected Result:**
- Basic `ls`: no output (empty)
- `ls -a`: should show `.` and `..` only

---

### 13. Permission Denied Directory

**Test Command:**
```bash
sudo mkdir restricted_dir
sudo chmod 000 restricted_dir
./ls restricted_dir
```

**Expected Result:**
- Prints error: `ls: cannot open directory 'restricted_dir': Permission denied`
- Exit code should be non-zero

**Cleanup:**
```bash
sudo chmod 755 restricted_dir
sudo rmdir restricted_dir
```

---

### 14. Symbolic Link Handling

**Test Command:**
```bash
./ls -l broken_link
./ls -lF broken_link
```

**Expected Result:**
- Should show the symlink itself, not try to follow it
- Broken links should be displayed (may show in different color/style)
- `-F` should append `@` to symlink names

---

### 15. Different File Types in Long Format

**Test Command:**
```bash
./ls -l
```

**Expected Result:**
- First character of permissions indicates file type:
  - `-` regular file
  - `d` directory  
  - `l` symbolic link
  - `p` named pipe (FIFO)
  - `s` socket
  - `b` block device
  - `c` character device

---

### 16. Flag Variations and Combinations

**Test Commands:**
```bash
./ls -l -a -F    # Separate flags
./ls -laf        # Combined short flags
./ls -fal        # Different order
./ls -F -a -l    # Mixed order
```

**Expected Result:**
- All variations should produce identical output
- Flag parsing should handle different combinations and orders

---

### 17. Current and Parent Directory Listing

**Test Command:**
```bash
./ls .
./ls ..
./ls -a .
./ls -l ..
```

**Expected Result:**
- `.` should list current directory contents
- `..` should list parent directory contents
- Should work with all flag combinations

---

### 18. Large Directory Handling

**Test Command:**
```bash
# Create many files
for i in {1..50}; do touch "file_$i.txt"; done
./ls
./ls -l | wc -l
```

**Expected Result:**
- Should handle directories with many files
- Output should be properly formatted
- Performance should be reasonable

**Cleanup:**
```bash
rm file_*.txt
```

---

## Expected Output Format Examples

### Basic `ls`:
```
empty_file  file1.txt  file2.log  link_to_dir  link_to_file  normal_dir  script.sh
```

### `ls -F`:
```
empty_file  file1.txt  file2.log  link_to_dir@  link_to_file@  normal_dir/  script.sh*
```

### `ls -l` (sample):
```
total 16
-rw-r--r-- 1 user group    0 Jul 20 10:30 empty_file
-rw-r--r-- 1 user group    9 Jul 20 10:30 file1.txt
-rw-r--r-- 1 user group    9 Jul 20 10:30 file2.log
lrwxrwxrwx 1 user group   10 Jul 20 10:30 link_to_dir -> normal_dir
lrwxrwxrwx 1 user group    9 Jul 20 10:30 link_to_file -> file1.txt
drwxr-xr-x 2 user group 4096 Jul 20 10:30 normal_dir
-rwxr-xr-x 1 user group   24 Jul 20 10:30 script.sh
```

## Running the Tests

### Individual Test Execution
```bash
# Test basic functionality
echo "Testing basic ls:"
./ls
echo ""

# Test with flags
echo "Testing ls -l:"
./ls -l
echo ""

echo "Testing ls -a:"
./ls -a
echo ""

echo "Testing ls -F:"
./ls -F
```

### Automated Test Script
```bash
#!/bin/bash
echo "Running ls command tests..."

# Setup test environment
mkdir -p ls_test_env
cd ls_test_env
echo "test" > testfile.txt
mkdir testdir
echo "hidden" > .hidden
chmod +x testfile.txt

echo "1. Testing basic ls..."
./ls

echo "2. Testing ls -a..."
./ls -a

echo "3. Testing ls -l..."
./ls -l

echo "4. Testing ls -F..."
./ls -F

echo "5. Testing ls -la..."
./ls -la

echo "6. Testing ls -lF..."
./ls -lF

echo "7. Testing ls -laF..."
./ls -laF

# Cleanup
cd ..
rm -rf ls_test_env
echo "Tests completed."
```

## Verification Checklist

- [ ] Basic `ls` shows only visible files
- [ ] `-a` flag shows hidden files including `.` and `..`
- [ ] `-l` flag shows detailed file information (permissions, size, date, etc.)
- [ ] `-F` flag adds appropriate type indicators (`/`, `*`, `@`, etc.)
- [ ] Flag combinations work correctly (`-la`, `-lF`, `-aF`, `-laF`)
- [ ] Different flag syntaxes work (`-l -a`, `-la`, `-al`)
- [ ] Handles non-existent files with appropriate error messages
- [ ] Handles permission denied scenarios
- [ ] Correctly displays different file types (regular, directory, symlink, etc.)
- [ ] Symlinks are properly identified and displayed
- [ ] Empty directories are handled correctly

## Cleanup

After running tests, clean up the test environment:

```bash
cd ..
rm -rf ls_test_dir
```

## Notes

- Replace `./ls` with the actual path to your `ls` implementation
- Output formatting may vary between implementations but core information should be consistent
- Date/time formats may differ based on system locale
- File ownership and permissions will reflect the user running the tests
- Some special file types (FIFOs, sockets) may not be available on all systems
- Symlink creation may require appropriate permissions

## Contributing

If you discover additional test cases or edge cases, please document them following the same format:

1. Clear test command
2. Expected result with specific output examples
3. Verification steps
4. Any special setup or cleanup requirements