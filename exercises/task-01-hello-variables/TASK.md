# Task 01 — Hello Variables

## Goal
Create your first **String** variables (text values) using correct Java syntax.

## What is a String?
A String is text. In Java you must write the type `String` before the name:

```java
String city = "Havana";
```

## Correct syntax

```java
String firstName = "Luis";
```

Wrong (do not use):

```java
let firstName = "Luis";   // JavaScript — not for this class
var firstName = "Luis";   // not the style we use here
```

## Where to work
Open this file and write your code there:

- `Starter.java`

Look for comments that start with `TODO`.
Write your variables **inside** the `main` method.

## Steps

1. Create `String firstName = "...";` with your first name.
2. Create `String lastName = "...";` with your last name.
3. Create `String favoriteColor = "...";` with a color name.
4. Do **not** delete the `System.out.println` lines at the bottom.

## Done when
- You have 3 String variables using `String name = "...";`
- Running the program prints your values (no errors).

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
cd student-tasks\task-01-hello-variables
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
