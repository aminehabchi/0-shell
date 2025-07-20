# Test Cases for `cp` Command (Without Flags)

This document provides comprehensive test cases for verifying the basic functionality of a `cp` (copy) command implementation. These tests cover standard behavior without using any command-line flags.

## Prerequisites

- A working `cp` command implementation (referenced as `./cp` in examples)
- Unix-like shell environment (bash, zsh, etc.)
- Basic file system permissions for testing

## Test Cases

### 1. Copy a Single File to a New File

**Test Command:**
```bash
echo "hello" > a.txt
./cp a.txt b.txt
```

**Expected Result:**
- `b.txt` is created and contains the same content as `a.txt`
- `a.txt` remains unchanged

**Verification:**
```bash
cat a.txt  # Should output: hello
cat b.txt  # Should output: hello
```

---

### 2. Copy Multiple Files into an Existing Directory

**Test Command:**
```bash
mkdir folder
echo "file1" > f1.txt
echo "file2" > f2.txt
./cp f1.txt f2.txt folder
```

**Expected Result:**
- `folder/f1.txt` and `folder/f2.txt` exist with correct contents

**Verification:**
```bash
cat folder/f1.txt  # Should output: file1
cat folder/f2.txt  # Should output: file2
```

---

### 3. Copy File into Existing Directory

**Test Command:**
```bash
mkdir dir
echo "data" > file.txt
./cp file.txt dir
```

**Expected Result:**
- `dir/file.txt` exists and matches the source

**Verification:**
```bash
cat dir/file.txt  # Should output: data
```

---

### 4. Source File Does Not Exist

**Test Command:**
```bash
./cp nofile.txt out.txt
```

**Expected Result:**
- Prints error: `cp: cannot stat 'nofile.txt': No such file or directory`
- Exit code should be non-zero

---

### 5. Destination Directory Does Not Exist

**Test Command:**
```bash
echo "test" > file.txt
./cp file.txt no_such_dir
```

**Expected Result:**
- Prints error: `cp: target 'no_such_dir' is not a directory`
- Exit code should be non-zero

---

### 6. Copy Directory (Should Fail)

**Test Command:**
```bash
mkdir mydir
./cp mydir copydir
```

**Expected Result:**
- Prints error: `cp: -r not specified; omitting directory 'mydir'`
- Exit code should be non-zero

---

### 7. Destination File Already Exists (Overwrite)

**Test Command:**
```bash
echo "A" > src.txt
echo "B" > dst.txt
./cp src.txt dst.txt
```

**Expected Result:**
- `dst.txt` is overwritten with contents of `src.txt`

**Verification:**
```bash
cat dst.txt  # Should output: A
```

---

### 8. Copy File to Itself

**Test Command:**
```bash
echo "x" > same.txt
./cp same.txt same.txt
```

**Expected Result:**
- Should do nothing or print a warning (implementation-dependent)
- File content should remain unchanged

---

### 9. Copy with Invalid Source Path

**Test Command:**
```bash
./cp "" dest.txt
```

**Expected Result:**
- Prints error: `cp: invalid source path ''`
- Exit code should be non-zero

---

### 10. Permission Denied on Destination

**Test Command:**
```bash
echo "secret" > secret.txt
sudo mkdir /root/testdir
./cp secret.txt /root/testdir
```

**Expected Result:**
- Prints error related to permission denied
- Exit code should be non-zero

**Note:** This test requires root privileges and may not work in all environments.

---

### 11. Copy Empty File

**Test Command:**
```bash
touch empty.txt
./cp empty.txt copy.txt
```

**Expected Result:**
- `copy.txt` is created and is also empty

**Verification:**
```bash
ls -la empty.txt copy.txt  # Both should have size 0
```

---

### 12. Copy Binary File

**Test Command:**
```bash
cp /bin/ls ls_copy
./cp ls_copy ls_copy2
cmp ls_copy ls_copy2  # Should return nothing (files identical)
```

**Expected Result:**
- Binary file is copied correctly
- `cmp` command should return no output (indicating files are identical)

---

