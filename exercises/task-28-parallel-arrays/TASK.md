# Task 28 — Parallel Arrays (Names + Scores)

## Goal
Use two arrays together: same index = same person.

## Idea

```
index:     0       1       2
names:   "Ana"   "Luis"  "Maria"
scores:   90      70      85
```

So `names[1]` and `scores[1]` both belong to Luis.

This is a beginner way to group data before learning classes / Firebase documents.

## Loop choice

Typical to print every pair: `for`.
Finding the best can be `for` or `while` — both correct.
This task: **`for`** to print, **`while`** to find the highest score.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `names` with 3 names.
2. Create int[] `scores` with 3 scores (same order as the names).
3. Print each student with a **`for`** loop.
4. Start with the first person as the current best (name + score).
5. Find the highest score with a **`while`** loop.

## Done when
The loop prints all pairs, and the best student is Ana with 90 (for this sample).
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
cd student-tasks\task-28-parallel-arrays
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
