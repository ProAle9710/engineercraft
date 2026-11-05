# Task 39 — Fix Scores That Are Too High

## Goal
Loop an int[] and change any value greater than 100 to 100.

## Where to work
- File: `Starter.java`

## Steps

1. Create int[] `scores` that includes some values above 100 and some below 60.
2. Use a **`for`** loop and cap values above 100.
3. Create int `passCount` starting at 0.
4. Use a **`while`** loop to count how many scores are >= 60.
5. Create int `failCount` from length minus passCount.
6. Create String `summary` with concatenation.
7. Create String `light` with a ternary (GREEN if passCount >= failCount, else RED).

## Done when
No printed score is above 100, and pass/fail counts match the capped values.
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
cd student-tasks\task-39-cap-scores
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
