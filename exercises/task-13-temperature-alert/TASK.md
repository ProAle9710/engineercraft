# Task 13 — Temperature Alert Logic

## Goal
Practice logical thinking with temperatures.

## Rules
- If temperature is greater than `30` → alert `"Hot"`
- Else if temperature is less than `10` → alert `"Cold"`
- Else → alert `"Normal"`

## Where to work
- File: `Starter.java`

## Steps

1. Create String `sensorId`.
2. Create int `temperature`.
3. Create String `unit` with C.
4. Complete `if / else if / else` for `alertMessage` (Hot / Cold / Normal).
5. Create boolean `needsAttention` using OR (`||`): true if hot or cold.

## Done when
Test with temperatures like `35`, `5`, and `22`.

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
cd student-tasks\task-13-temperature-alert
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
