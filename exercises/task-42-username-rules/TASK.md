# Task 42 — Username Rules

## Goal
Check several rules on one String, then decide with if/else (several actions).

## Rules
- Length must be at least 5.
- First character must not be a space.
- It must not be equal to admin (use `.equals`).

## Where to work
- File: `Starter.java`

## Steps

1. Create String `username`.
2. Create int `size` using `.length()`.
3. Create char `first` using `.charAt(0)` (if size is 0, you cannot call charAt — use if/else and put `'?'` when empty).
4. Create boolean `longEnough`.
5. Create boolean `noSpace` (first is not `' '`).
6. Create boolean `notAdmin` (not equal to admin).
7. Create boolean `valid` using `&&` on the three booleans.
8. Use if/else:
   - If valid: message OK and level 1
   - Else: message REJECTED and level 0
9. Create a long String `log` (imagine this text would go to Firebase later).

## Done when
Test three usernames: a valid one, admin, and a short one.
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
cd student-tasks\task-42-username-rules
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
