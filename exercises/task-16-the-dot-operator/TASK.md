# Task 16 — The Dot ( . ) on a Variable

## Goal
Learn that a variable is not only a box with a value.
In Java, some types (like `String`) can also **do actions**.
You call those actions with a **dot** `.`

## New idea: the dot

Look at this:

```java
String name = "Ana";
int size = name.length();
```

What happens?

1. `name` stores the text `"Ana"`.
2. The **dot** `.` means: "use something that belongs to this variable".
3. `length()` is an **action** (called a **method**) that counts the characters.
4. For `"Ana"`, `size` becomes `3`.

So: `variable.method()`

- Left of the dot = the variable
- Right of the dot = the action

## Important

- You write the parentheses `()` even when there is nothing inside: `length()`
- This is NOT math. The dot is not multiplication.
- Numbers like `int` do not use `.length()` the same way. This task is about `String`.

## Where to work
- File: `Starter.java`
- Write code only in the `TODO` sections inside `main`.

## Steps

1. Create a String `fullName` with your name (you already know how).
2. Create int `letterCount` using `.length()` on `fullName` (new — see the example above).
3. Create a String `greeting`.
4. Create int `greetingSize` using `.length()` on `greeting`.
5. Print is already done for you. Read the output and check the numbers make sense.

## Done when
You used the dot `.` at least twice with `.length()` and the program runs.
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
cd student-tasks\task-16-the-dot-operator
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
