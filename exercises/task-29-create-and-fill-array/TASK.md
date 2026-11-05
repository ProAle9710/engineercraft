# Task 29 — Create an Empty Array and Fill It

## Goal
Create an array with a fixed size, then put values into each index.

## Two ways to create arrays

Already known:

```java
int[] a = {1, 2, 3};
```

New way (size first, values later):

```java
int[] a = new int[3]; // 3 empty slots (start as 0)
a[0] = 10;
a[1] = 20;
a[2] = 30;
```

For Strings:

```java
String[] ids = new String[3]; // slots start as null
ids[0] = "user_1";
```

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `userIds` with size 3 using `new` (new syntax — see above).
2. Fill the 3 slots with user_1, user_2, user_3 (practice of changing indexes).
3. Create int[] `levels` with size 3 using `new`.
4. Fill levels with 1, 2, 3.
5. Print all userIds with a **`for`** loop (typical for a whole array). Include the matching level.
6. Print them again with a **`while`** loop (also correct).

## Firebase idea
Later, a list of user ids is something you might read from Firebase.
Today you only build the array by hand.

## Done when
All slots are filled and printed.
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
cd student-tasks\task-29-create-and-fill-array
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
