# Task 05 — Change Values and Concatenate Strings

## Goal
1) Change a variable after creating it.
2) Join Strings with `+` to build a longer text.

## Change a value

```java
int lives = 3;
lives = 2; // now 2
```

## Concatenation (join text)

```java
String a = "Welcome";
String b = " player";
String finalText = a + b; // "Welcome player"
```

You can join many pieces:

```java
String full = "User: " + "Ana" + " | Level: " + 5;
// Result: "User: Ana | Level: 5"
```

Numbers can be concatenated into a String too. Java turns them into text.

## Where to work
- File: `Starter.java`

## Steps

1. Create String `message` with Hello (practice).
2. Create int `points` as 0 (practice).
3. Change `message` to Welcome (new — reassign, no type again).
4. Change `points` to 10.
5. Create String `finalText` by concatenating `message` and ` player`.
6. Create a longer String `longText` by concatenating `finalText`, has, `points`, and more text.
   Expected idea: Welcome player has 10 points today.

## Done when
`longText` is one longer String built with several `+` concatenations.
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
cd student-tasks\task-05-rename-and-reuse
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
