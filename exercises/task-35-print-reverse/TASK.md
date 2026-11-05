# Task 35 — Print an Array in Reverse

## Goal
Loop from the last index down to 0.

## Loop idea

Start `i` at `items.length - 1`.
Keep going while `i >= 0`.
After each turn, `i--` (minus one).

You already know `i++`. `i--` is the same idea, backwards.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `steps` with 4 short texts (step1, step2, ...).
2. Print them in normal order with a **`for`** loop (typical).
3. Print them in reverse with a **`while`** loop going backwards (`i--`). Also correct with `for`; you must use `while` here.
4. Create String `lastStep` from the last index.
5. Create String `firstStep` from index 0.

## Done when
Reverse order shows the last item first.
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
cd student-tasks\task-35-print-reverse
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
