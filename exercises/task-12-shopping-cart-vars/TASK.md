# Task 12 — Shopping Cart with Ternary + Long String

## Goal
Compute a total, choose a value with a **ternary**, and build a long receipt String.

## Ternary for a number

```java
int finalTotal = hasDiscount ? subtotal - 2 : subtotal;
```

Same idea as if/else, but in one line, and it **returns a value**.

## Where to work
- File: `Starter.java`

## Steps

1. Create String `productName`.
2. Create int `unitPrice`.
3. Create int `quantity`.
4. Create int `subtotal` (price times quantity).
5. Create boolean `hasDiscount`.
6. Create int `finalTotal` with a **ternary** (discount of 2 if hasDiscount, else subtotal).
   The example above in this file is the new idea — write it yourself.
7. Create String `discountLabel` with a ternary: YES or NO.
8. Create String `receipt` by concatenating several pieces (use `\n` for new lines).
   Include product, unit, qty, subtotal, discount, total.

## Tip
`\n` means new line inside a String.

## Done when
`finalTotal` and `discountLabel` use ternaries, and `receipt` is one long String.
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
cd student-tasks\task-12-shopping-cart-vars
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
