# Task 40 — Build Many Firestore Paths

## Goal
From an array of document ids, build paths like `users/u1`.

## Where to work
- File: `Starter.java`

## Steps

1. Create String `collectionName` with users.
2. Create String[] `ids` with 4 ids.
3. Create String[] `paths` with `new String[ids.length]`.
4. Fill each `paths[i]` with a **`for`** loop (concatenation).
5. Create String `wantedId`.
6. Search with a **`while`** loop and set boolean `exists` (typical for search; `for` is also correct).
7. Create String `wantedPath` with a ternary:
   if exists, build the path; else use NOT_FOUND.
8. Print all paths with a **`for`** or **`while`** (your choice — both are correct), then print wantedPath.

## Done when
Four paths print, and wantedPath is correct for both a real id and a missing id.
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
cd student-tasks\task-40-firestore-paths
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
