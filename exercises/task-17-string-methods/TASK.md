# Task 17 — Strings Are Made of Characters (char[])

## Goal
Understand that a `String` is like an **array of characters**.
You will use `.charAt(index)` and also `.toCharArray()`.

## Big idea

```text
String word = "Java";
index:         0 1 2 3
chars:         J a v a
```

## Two ways to read characters

### 1) One character with charAt

```java
char first = word.charAt(0); // 'J'
```

### 2) Convert the whole String into a real char array

```java
char[] letters = word.toCharArray();
// letters[0] is 'J'
// letters[1] is 'a'
// letters.length is 4
```

Now you can treat the text almost like a normal array.

## char vs String

```java
char letter = 'J';      // ONE character, single quotes '
String text = "J";      // text, double quotes "
```

## Where to work
- File: `Starter.java`

## Steps

1. Create String `word` with the text Java.
2. Create int `size` using `.length()` (practice from Task 16).
3. Create char `first` using `.charAt(0)` (new — see example above).
4. Create char `last` using `.charAt` on the last index (length - 1).
5. Create `char[] letters` using `.toCharArray()` (new — see example above).
6. Create char `secondFromArray` from index 1 of `letters`.
7. Create int `arraySize` from `letters.length` (no parentheses).

## Done when
You used both `charAt` and `toCharArray`, and you can say:
"a String can become a `char[]`."
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
cd student-tasks\task-17-string-methods
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
