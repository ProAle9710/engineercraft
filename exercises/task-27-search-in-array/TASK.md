# Task 27 — Search for a Value in an Array

## Goal
Check if a name exists inside a `String[]` using a loop and `.equals`.

## Logic

```
found starts as false
for each name in the array:
    if that name equals the target:
        found = true
```

```java
boolean found = false;
for (int i = 0; i < names.length; i++) {
    if (names[i].equals(target)) {
        found = true;
    }
}
```

Notice: `names[i]` is a String, so you can use the **dot** on it:
`names[i].equals(target)`

## Loop choice

Typical for search: `while` (`!found && i < names.length`) so you can stop when you find it.
A `for` that walks the whole array is also **correct**.
This task: search with **`while`**, then print all names with **`for`**.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `names` with 4 names.
2. Create String `target` (later try a name that is not in the list).
3. Create boolean `found` starting as false.
4. Search with a **`while`** loop using `.equals`.
5. Print every name with a **`for`** loop.
6. Set String `message` with if/else: User found / User not found.

## Done when
`"Maria"` finds true, and a missing name finds false.
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
cd student-tasks\task-27-search-in-array
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
