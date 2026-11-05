# Task 34 — Join Names into One Long String

## Goal
Loop an array and build one String with commas: Ana, Luis, Maria

## Trick
Do not put a comma after the last name.

Idea: if this is not the last index, add `", "` after the name.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `names` with 4 names.
2. Create String `list` as empty text.
3. Use a **`for`** loop to build `list` with commas (typical for every name).
3b. Print each name with a **`while`** loop (also correct).
4. Create int `howMany` from the array length.
5. Create String `header` by concatenation (how many people).
6. Create String `full` = header + new line + list.

## Done when
The printed list has commas between names, but not after the last name.
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
cd student-tasks\task-34-join-names
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
