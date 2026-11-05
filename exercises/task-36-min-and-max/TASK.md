# Task 36 — Smallest and Largest Number

## Goal
Walk an int[] and keep the smallest and the largest value.

## Idea
Start both `min` and `max` as `numbers[0]`.
Then loop from index 1.
If the current number is smaller, update `min`.
If it is larger, update `max`.

## Where to work
- File: `Starter.java`

## Steps

1. Create int[] `numbers` with at least 5 values (mix high and low).
2. Create int `min` and int `max` from index 0.
3. Use a **`for`** loop to update min / max (typical for every number).
3b. Print each number with a **`while`** loop.
4. Create int `spread` as max minus min.
5. Create String `summary` by concatenation.
6. Create String `alert` with a ternary (spread greater than 50 or not).

## Done when
Check with a calculator that min, max, and spread are correct.
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
cd student-tasks\task-36-min-and-max
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
