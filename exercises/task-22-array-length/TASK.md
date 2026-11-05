# Task 22 — array.length (How Many Items?)

## Goal
Learn how to ask an array how many elements it has.

## Important difference

| Code | Used on | Meaning |
|------|---------|---------|
| `text.length()` | String | method with `()` |
| `items.length` | array | property **without** `()` |

```java
String[] items = {"A", "B", "C"};
int count = items.length; // 3
```

Notice: **no parentheses** after `length` for arrays.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `users` with 4 values (practice).
2. Create int `userCount` using `.length` on the array (new — no `()`).
3. Create int[] `grades` with 3 values.
4. Create int `gradeCount` using `.length`.
5. Create boolean `hasThreeGrades` (is gradeCount equal to 3?).

## Done when
You used `.length` (no `()`) on arrays, and the program runs.
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
cd student-tasks\task-22-array-length
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
