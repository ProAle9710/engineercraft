# Task 33 — First and Last Character

## Goal
Combine `length()`, `charAt`, booleans, and a ternary.

## Where to work
- File: `Starter.java`

## Steps

1. Create String `code` (example idea: a short product code).
2. Create int `size` from the String length.
3. Create boolean `tooShort` (true if size is less than 2).
4. If `tooShort` is true, set char `first` and char `last` to `'?'`.
   Else, read the real first and last characters.
   Use if/else here (two assignments — not a good ternary).
5. Create boolean `sameEnds` (first equals last). Use `==` for char.
6. Create String `message` with a ternary.

## Done when
Try codes like ABBA (same ends) and JAVA (different ends). Also try a 1-letter String.
## How to run

Follow these steps carefully. Do not run the commands from a random folder.

### 1) Open a console (terminal)

- **VS Code**: menu **Terminal → New Terminal** (or the shortcut shown in that menu)
- **IntelliJ IDEA**: menu **View → Tool Windows → Terminal** (or the Terminal tab at the bottom)
- Or open **PowerShell** / **Command Prompt** on Windows

### 2) Go to THIS task folder

You must be inside the folder that contains `Starter.java`.

Example (change the path if needed):

```bash
cd student-tasks\task-33-first-last-char
```

Check with:

```bash
dir
```

You should see `Starter.java` and `TASK.md`.

### 3) Compile

```bash
javac Starter.java
```

This creates `Starter.class`. If you see errors, fix your code and compile again.

### 4) Run

```bash
java Starter
```

- Correct: `java Starter`
- Wrong: `java Starter.java`
- Wrong: running this from another folder
