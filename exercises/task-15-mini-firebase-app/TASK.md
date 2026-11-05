# Task 15 — Mini Firebase App (Put It Together)

## Goal
Combine Java variables + simple logic + Firebase beginner concepts.

## Save rule
A note can be saved only if:
- `isLoggedIn` is `true` **and**
- `noteText` is not an empty String `""`

## Where to work
- File: `Starter.java`

## Steps

### A) Firebase config (demo)
1. Create String `projectId` with class-demo.
2. Create boolean `isDemoMode` as true.

### B) User
3. Create String `userName`.
4. Create boolean `isLoggedIn`.

### C) Note document
5. Create String `noteId`.
6. Create String `noteText`.
7. Create String `notePath` by concatenating notes/ and noteId.

### D) Logic
8. Create boolean `canSave` from `isLoggedIn`.
9. Set String `saveMessage` with if/else:
   - If can save → Note ready to save to Firebase
   - Else → Cannot save yet

## Done when
All variables use correct Java types and your logic matches the save rule.

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
cd student-tasks\task-15-mini-firebase-app
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
