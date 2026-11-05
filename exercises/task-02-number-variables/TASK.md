# Task 02 — Number Variables

## Goal
Create **int** (whole number) variables and use them in a tiny calculation.

## Correct syntax
Numbers do **not** use quotes. Write the type before the name:

```java
int age = 18;
int heightCm = 170;
```

## Where to work
- File: `Starter.java`
- Write code only in the `TODO` sections inside `main`.

## Steps

1. Create `int age = ...;`
2. Create `int heightCm = ...;`
3. Create `int shoesOwned = ...;`
4. Create `int totalItems = shoesOwned + 2;` (you bought 2 more shoes).

## Tip
You can create a new value from other variables:

```java
int total = a + b;
```

## Done when
- All variables use `int name = value;`
- `totalItems` equals `shoesOwned + 2`.

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
cd student-tasks\task-02-number-variables
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
