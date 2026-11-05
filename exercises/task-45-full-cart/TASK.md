# Task 45 — Full Cart (Qty x Price)

## Goal
Parallel arrays: products, prices, quantities.
Compute line totals, a cart total, discount, and a long receipt.

## Where to work
- File: `Starter.java`

## Steps

1. Create String[] `products` with 3 names.
2. Create int[] `prices` with 3 unit prices.
3. Create int[] `qty` with 3 quantities.
4. Create int[] `lineTotal` with `new int[3]`.
5. Fill `lineTotal[i]` with a **`for`** loop (price * qty).
6. Create int `subtotal` as the sum of lineTotal.
7. Create boolean `hasCoupon`.
8. Create int `finalTotal` with a ternary (if coupon, subtotal - 5, but never below 0 — if that needs two steps, use if/else).
9. Create String `receipt` by growing it in a **`while`** loop, then add subtotal and finalTotal.

## Done when
Each line total is correct, and the receipt is one long String.
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
cd student-tasks\task-45-full-cart
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
