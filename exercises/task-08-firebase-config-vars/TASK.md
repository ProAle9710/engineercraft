# Task 08 — Firebase Config as Variables

## Goal
Store fake Firebase configuration values in **String** and **boolean** variables.
You do **not** need a real Firebase account.

## Correct syntax examples

```java
String apiKey = "DEMO_API_KEY_123";
boolean isDemoMode = true;
```

## Where to work
- File: `Starter.java`

## Steps

Write each variable yourself (this is practice of String and boolean).

1. Create String `apiKey` with the demo text `DEMO_API_KEY_123`.
2. Create String `authDomain` with `demo-project.firebaseapp.com`.
3. Create String `projectId` with `demo-project`.
4. Create String `storageBucket` with `demo-project.appspot.com`.
5. Create String `appId` with `1:000000:web:abcdef`.
6. Create boolean `isDemoMode` as true.

## Security note
Use **demo / fake** values only in class.

## Done when
All config variables exist with correct Java types.

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
cd student-tasks\task-08-firebase-config-vars
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
