# Task 21 — Array Indexes (Positions)

## Goal
Read one value from an array using its **index** (position).

## Indexes start at 0

```java
String[] fruits = {"Apple", "Banana", "Cherry"};
// index:              0         1         2
```

| Code | Result |
|------|--------|
| `fruits[0]` | `"Apple"` |
| `fruits[1]` | `"Banana"` |
| `fruits[2]` | `"Cherry"` |

Common beginner mistake: thinking the first item is `[1]`. In Java it is `[0]`.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `days` with 5 short day names (practice of Task 19).
2. Create String `firstDay` from index 0 (new — see table).
3. Create String `thirdDay` from index 2.
4. Create String `lastDay` from the last index.
5. Create int[] `points` with 3 numbers (practice of Task 20).
6. Create int `middle` from index 1 of `points`.

## Done when
You can explain out loud: "index 0 is the first element".
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
cd student-tasks\task-21-array-indexes
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
