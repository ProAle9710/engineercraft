# Task 19 — Your First Array (String[])

## Goal
Learn what an **array** is and create a `String` array.

## What is an array?

One variable that stores **many values** of the same type, in a list with positions.

Think of boxes in a row:

```
index:   0        1        2
value: "Ana"   "Luis"   "Maria"
```

## Correct syntax

```java
String[] names = {"Ana", "Luis", "Maria"};
```

Read it as:

- `String[]` = an array of Strings
- `names` = the variable name
- `{ ... }` = the values inside

## Where to work
- File: `Starter.java`

## Steps

1. Create `String[] students` with 3 student names.
2. Create `String[] colors` with 3 color names.
3. Create `String[] firebaseCollections` with these values:
   `"users"`, `"notes"`, `"grades"`
   (Just names of collections — we are not connecting to Firebase yet.)

## Done when
You created 3 arrays using `String[] name = { ... };`
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
cd student-tasks\task-19-first-string-array
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
