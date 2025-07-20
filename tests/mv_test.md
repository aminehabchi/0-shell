
# 📦 mv – Test Suite (No Flags)

A set of tests to verify a custom `mv` command built in Rust (no flag support).

---

### 1. 📝 Rename a File

**Command:**
```bash
echo "hello" > test1.txt
./mv test1.txt test2.txt
```

**Expected:**
- `test1.txt` is removed.
- `test2.txt` exists and contains `"hello"`.

---

### 2. 📂 Move File into Existing Directory

**Command:**
```bash
mkdir folder
echo "world" > test.txt
./mv test.txt folder
```

**Expected:**
- `folder/test.txt` exists and contains `"world"`.

---

### 3. 📁 Rename a Directory

**Command:**
```bash
mkdir mydir
./mv mydir newdir
```

**Expected:**
- `mydir` is renamed to `newdir`.

---

### 4. 📦 Move Directory into Another Directory

**Command:**
```bash
mkdir dir1
mkdir dir2
./mv dir1 dir2
```

**Expected:**
- `dir2/dir1` exists.

---

### 5. ❌ Move Non-existent File

**Command:**
```bash
./mv not_exist.txt file.txt
```

**Expected:**
- Error message like: `No such file or directory`.

---

### 6. ⚠️ Move File to Existing File (Overwrite Behavior)

**Command:**
```bash
echo "A" > file1
echo "B" > file2
./mv file1 file2
```

**Expected:**
- Should overwrite `file2`, or fail — depends on implementation.

---

### 7. 📛 Move File into Non-existent Directory

**Command:**
```bash
echo "test" > file.txt
./mv file.txt no_such_dir
```

**Expected:**
- Error message like: `No such file or directory`.

---

### 8. 🔁 Move File to Itself

**Command:**
```bash
echo "x" > same.txt
./mv same.txt same.txt
```

**Expected:**
- Should do nothing or print a warning.
