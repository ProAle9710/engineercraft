# Task 14 — Student Grade with Ternary

## Goal
Decide pass / fail using a **boolean** and a **ternary** (`? :`).
Also build a longer report String with concatenation.

## Ternary (short if/else that gives a value)

```java
String status = didPass ? "Pass" : "Fail";
```

Read it as: `condition ? valueIfTrue : valueIfFalse`

## Where to work
- File: `Starter.java`

## Steps

Practice. Do **not** copy a full finished line — write it.

1. Create String `studentName`.
2. Create String `courseName`.
3. Create int `score` (0 to 100).
4. Create int `passingScore` as 60.
5. Create boolean `didPass` using `>=`.
6. Create String `status` with a ternary: Pass or Fail.
7. Create String `light` with a ternary: GREEN or RED.
8. Create a long String `report` by concatenating name, course, score, status, light.
9. Create String `firebaseCollection` with the text grades.

## Done when
`status` and `light` come from ternaries, and `report` is one long String.
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
cd student-tasks\task-14-student-grade
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
