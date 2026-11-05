# Task 46 — Mini Class Report (Put Level 3 Together)

## Goal
Build a small "would save to Firebase" report using everything you know:
arrays, loops, `.equals`, concatenation, booleans, ternary, if/else.

## Story
You have users in collection `grades`. Each user has a name and a score.
You will search one user, compute class stats, and build one long document text.

## Where to work
- File: `Starter.java`

## Steps

1. Create String `collectionName` (grades).
2. Create String[] `ids`, String[] `names`, int[] `scores` (length 4, same order).
3. Create String `searchId`.
4. Search the id with a **`while`** loop. Keep the matching name and score (or found = false). A `for` would also be correct.
5. Create String `docPath` with a ternary (collection/id or NOT_FOUND).
6. Compute class average and passCount with a **`for`** loop.
7. Create boolean `canPublish` (found AND average >= 60).
8. If canPublish, create `saveMessage` ready to save; else explain what is missing (if/else with more than one sentence is OK — that is why if/else exists).
9. Create String `document` as one long text: path, search result, average, passCount, saveMessage.

## Done when
Changing searchId and scores changes path, found, and canPublish in a way that makes sense.
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
cd student-tasks\task-46-firebase-class-report
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
