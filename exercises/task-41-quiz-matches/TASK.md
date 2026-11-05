# Task 41 — Quiz: How Many Answers Are Correct?

## Goal
Two String[] of the same length: `answers` (student) and `key` (correct).
Count matches with `.equals`.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `key` with 5 short answers (example: A, B, C, A, D).
2. Create String[] `answers` with 5 student answers (some wrong).
3. Create int `correct` starting at 0.
4. Use a **`for`** loop: if answers[i] equals key[i], add 1.
4b. Print each pair (key vs answer) with a **`while`** loop.
5. Create int `wrong` from length minus correct.
6. Create int `percent` as correct * 100 / key.length (integer division is OK).
7. Create String `grade` with if / else if / else:
   - percent >= 80 → A
   - percent >= 60 → B
   - else → F
8. Create a long String `report`.

## Done when
correct + wrong equals 5, and grade matches the percent rule.
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
cd student-tasks\task-41-quiz-matches
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
