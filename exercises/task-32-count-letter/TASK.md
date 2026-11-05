# Task 32 — Count One Letter in a Word

## Goal
Use `toCharArray()`, a loop, and `if` to count how many times a letter appears.

## Reminder (chars)

```java
char letter = 'a';
if (letters[i] == letter) { ... }
```

For `char`, use `==` (not `.equals`). Single quotes: `'a'`.

## Loop choice

Typical for every character: `for`. `while` is also correct.
This task: **`for`** to count, **`while`** to print each char.

## Where to work
- File: `Starter.java`

## Steps

Write the code yourself.

1. Create String `word` with at least 6 letters.
2. Create char `target` (the letter you will count).
3. Create `char[] letters` from the word.
4. Create int `count` starting at 0.
5. Use a **`for`** loop. If the current char equals `target`, add 1 to `count`.
6. Use a **`while`** loop to print each character.
7. Create String `report` by concatenating the word, the letter, and the count.
8. Create String `status` with a ternary: found vs not found (`count > 0`).

## Done when
If the word is banana and the letter is a, count is 3.
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
cd student-tasks\task-32-count-letter
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
