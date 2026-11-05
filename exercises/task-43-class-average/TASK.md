# Task 43 — Class Average and Status

## Goal
Sum, average, count passes, and build a report. All in one program.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `names` with 5 students.
2. Create int[] `scores` with 5 scores.
3. Create int `total` and sum the scores with a **`while`** loop.
4. Create int `average` (total / length).
5. Create int `passCount`.
6. Use a **`for`** loop: print name + score + a ternary Pass/Fail for that row.
7. Create boolean `classOk` (average >= 70 AND passCount >= 3).
8. Create String `banner` with a ternary.
9. Create String `report` that includes average, passCount, banner.

## Done when
Each row prints, and classOk follows both rules at the same time.
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
cd student-tasks\task-43-class-average
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
