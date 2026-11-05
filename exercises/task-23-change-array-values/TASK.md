# Task 23 — Change a Value Inside an Array

## Goal
Learn that array slots can be updated after you create the array.

## Idea

```java
String[] names = {"Ana", "Luis", "Maria"};
names[1] = "Carlos"; // now position 1 is "Carlos"
```

You do **not** write the type again when changing one slot.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `team` with 3 names (practice).
2. Change the first slot to Alex (new — see the idea above).
3. Change the last slot to Chris.
4. Create int[] `lives` with three 3s.
5. Change the middle slot to 2.
6. Create String `leader` from index 0 of `team`.

## Done when
Printed values show your changes.
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
cd student-tasks\task-23-change-array-values
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
