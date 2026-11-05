# Task 30 — Build Long Strings + Ternary

## Goal
Practice **concatenating** Strings (`+`) to build a long text,
and use a **ternary** for the final message.

## Concatenation

`+` joins text pieces into a bigger String:

```java
String line = "Product: " + "Pen" + " - price: " + 5;
// Result: "Product: Pen - price: 5"
```

You can also grow a String step by step:

```java
String report = "";
report = report + "Line 1\n";
report = report + "Line 2\n";
```

(`\n` means a new line.)

## Ternary reminder

```java
String msg = canBuyAll ? "You can buy everything" : "Not enough money";
```

## Loop choice

Typical for the catalog walk: `for`.
Summing with `while` is also correct.
This task uses **both**.

## Where to work
- File: `Starter.java`

## Steps

Practice. Write every line yourself.

1. Create String[] `products` with 3 product names.
2. Create int[] `prices` with 3 prices (same order).
3. Create String `catalog` as empty text.
4. Use a **`for`** loop to grow `catalog` with concatenation (typical for every product).
5. Create int `total` at 0, then sum all prices with a **`while`** loop (also correct).
6. Create boolean `canBuyAll` (total at most 50).
7. Create String `checkoutMessage` with a ternary.
8. Create one long String `fullReport` (catalog + total + message).

## Done when
`fullReport` is one long String with catalog + total + message. Total should be `28`.
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
cd student-tasks\task-30-mini-array-shop
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
