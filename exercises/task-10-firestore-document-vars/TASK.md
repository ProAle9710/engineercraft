# Task 10 — One Firestore Document as Variables

## Goal
Represent one Firestore document using Java variables.

## Where to work
- File: `Starter.java`

## Steps

Write the variables yourself. Concatenate with `+` like in Task 05.

1. Create String `collectionName` with the text books.
2. Create String `documentId` with the text book_1.
3. Create String `title`.
4. Create String `author`.
5. Create int `pages`.
6. Create boolean `available`.
7. Create String `path` by joining collectionName, `/`, and documentId.
   Expected idea: books/book_1

## Done when
All fields use correct types and `path` looks like `books/book_1`.

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
cd student-tasks\task-10-firestore-document-vars
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
