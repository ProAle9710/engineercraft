# Task 26 — Count + Ternary (Short if)

## Goal
1) Count passing scores with a loop.
2) Learn the **ternary operator**: a short if/else that returns a value.

## Ternary = another way to write if/else

Normal if/else:

```java
String label;
if (didPass) {
    label = "Pass";
} else {
    label = "Fail";
}
```

Same idea with a ternary:

```java
String label = didPass ? "Pass" : "Fail";
```

Read it as:

```text
condition ? valueIfTrue : valueIfFalse
```

It is still about a **boolean** condition (`true` / `false`).

## Loop choice

Typical: `for` to visit every score.
`while` is also correct.
This task: **`for`** to count passes, **`while`** to print each score.

## Where to work
- File: `Starter.java`

## Steps

Practice. Write every line yourself.

1. Create int[] `scores` with 55, 80, 42, 90, 60.
2. Create int `passCount` starting at 0.
3. Use a **`for`** loop: if a score is at least 60, add 1 to `passCount`.
4. Use a **`while`** loop to print every score.
5. Create int `failCount` from length minus passCount.
6. Create boolean `mostlyPassing`.
7. Create String `summary` with a ternary (most passed vs most failed).
8. Create String `firstResult` with a ternary for scores[0] (Pass / Fail).

## Done when
`passCount` is `3`, `failCount` is `2`, and both ternary Strings are created.
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
cd student-tasks\task-26-count-with-if
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
