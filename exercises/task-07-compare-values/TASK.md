# Task 07 — Compare Values

## Goal
Compare two numbers and store the result in a boolean.

## Comparison operators
| Operator | Meaning | Example |
|----------|---------|---------|
| `>` | greater than | `5 > 3` → `true` |
| `<` | less than | `2 < 1` → `false` |
| `==` | equal to | `4 == 4` → `true` |

## Where to work
- File: `Starter.java`

## Steps

1. Create int `myScore`.
2. Create int `highScore`.
3. Create boolean `isNewRecord` using `>` (new — see the table).
4. Create boolean `isExactMatch` using `==`.
5. Set String `resultMessage` with if/else:
   - New record → New record!
   - Else → Keep practicing

## Done when
Change the numbers and check that the booleans and message make sense.

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
cd student-tasks\task-07-compare-values
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
