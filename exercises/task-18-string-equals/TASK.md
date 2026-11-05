# Task 18 — Compare Strings with .equals

## Goal
Learn the correct way to compare two `String` values in Java.

## Why not use == for text?

For beginners, remember this rule:

- Use `==` for numbers (`int`) and for `boolean`
- Use `.equals(...)` for `String` text

```java
String a = "yes";
String b = "yes";
boolean same = a.equals(b); // true
```

Read it like this: "ask variable `a` if it equals the value of `b`".

## Empty text

```java
boolean isEmpty = note.equals("");
```

`""` means a String with zero characters.

## Where to work
- File: `Starter.java`

## Steps

1. Create String `password`.
2. Create String `typed` (later change it to test).
3. Create boolean `isCorrect` using `.equals` (new — see example above).
4. Create String `note` (try empty text, then try text).
5. Create boolean `noteIsEmpty` using `.equals` with empty text.
6. Set String `status` with if/else:
   - Correct → Access granted
   - Else → Access denied

## Done when
You used `.equals` (not `==`) to compare Strings, and the if/else works.
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
cd student-tasks\task-18-string-equals
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
