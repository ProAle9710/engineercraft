# Task 06 — Simple Logic (if / else)

## Goal
Use a boolean variable to choose what message to show.

## Logic idea

```java
if (isRaining) {
    advice = "Take an umbrella";
} else {
    advice = "Enjoy the sun";
}
```

## Where to work
- File: `Starter.java`

## Steps

1. Create boolean `isRaining`.
2. Create String `advice` (start empty).
3. Inside the `if / else` block, set `advice` correctly.

## Done when
- If `isRaining` is `true`, advice is `"Take an umbrella"`.
- If `isRaining` is `false`, advice is `"Enjoy the sun"`.

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
cd student-tasks\task-06-simple-logic
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
