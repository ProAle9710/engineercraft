# Task 11 — Auth Status Logic

## Goal
Practice login status with booleans and `&&` (AND).

## Logical AND

```java
boolean canEnter = isLoggedIn && hasEmail;
```

## Where to work
- File: `Starter.java`

## Steps

1. Create boolean `isLoggedIn`.
2. Create boolean `hasEmail`.
3. Create String `userEmail` (empty text if hasEmail is false).
4. Create boolean `canEnterHome` using AND (`&&`) on the two booleans above.
5. Set String `screenMessage` with if/else:
   - If can enter → Welcome to Home
   - Else → Please log in

## Done when
Try different true/false combinations and check the message.

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
cd student-tasks\task-11-auth-status-logic
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
