# Task 44 — Copy Passing Students to a New Array

## Goal
1) Count how many pass.
2) Create a new String[] of that exact size.
3) Copy only the passing names into it.

## Two-loop pattern

Loop 1: count.
Then `String[] passed = new String[count];`
Loop 2: copy. You need a second index `p` that only grows when you copy.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `names` and int[] `scores` (same length, at least 5).
2. Create int `count` of scores >= 60 using a **`for`** loop.
3. Create String[] `passed` with `new String[count]`.
4. Create int `p` starting at 0.
5. Second loop must be **`while`**: if the student passes, put the name in `passed[p]` and add 1 to `p`. (`for` would also work.)
6. Print the passed array.
7. Create String `joined` by concatenating those names with spaces.
8. Create String `note` with a ternary (someone passed vs nobody passed).

## Done when
`passed.length` equals the number of passing students, and the names match.
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
cd student-tasks\task-44-copy-passing-names
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
