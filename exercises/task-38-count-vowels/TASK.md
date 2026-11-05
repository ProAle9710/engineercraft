# Task 38 — Count Vowels

## Goal
Turn a String into `char[]` and count vowels: a e i o u (lowercase only for this task).

## Hint
You can use several `||` in one if:

```java
if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
```

## Where to work
- File: `Starter.java`

## Steps

1. Create String `text` (use lowercase letters).
2. Create `char[] letters` from the text.
3. Create int `vowels` starting at 0.
4. Create int `other` starting at 0.
5. Use a **`for`** loop to count vowels vs other.
5b. Print each character with a **`while`** loop.
6. Create boolean `moreVowels` (vowels greater than other).
7. Create String `report` by concatenation.

## Done when
For "java" you should get 2 vowels (a, a) and 2 others (j, v).
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
cd student-tasks\task-38-count-vowels
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
