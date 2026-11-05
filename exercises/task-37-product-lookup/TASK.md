# Task 37 — Find a Product and Its Price

## Goal
Search in a String[] and, when you find the name, read the price from the same index in an int[].

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `products` with 4 product names.
2. Create int[] `prices` with 4 prices (same order).
3. Create String `wanted` (one of the names, then try a name that is missing).
4. Create boolean `found` as false.
5. Create int `priceFound` as 0.
6. Search with a **`while`** loop (typical when you may stop after finding). A `for` would also be correct.
6b. Print the full catalog with a **`for`** loop.
7. Create String `result` with if/else (found vs not found). Include the price when found.
8. Create String `tag` with a ternary: IN STOCK vs MISSING.

## Done when
A real product shows its price. A missing name shows not found.
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
cd student-tasks\task-37-product-lookup
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
