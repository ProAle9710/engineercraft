# Task 24 — Loops: `for` and `while`

## Goal
This is the lesson where you **learn loops**. Read it slowly.
You will use `for` and `while` to repeat work instead of copying the same lines.

## Why a loop?

If an array has many items, writing `names[0]`, `names[1]`, `names[2]`... by hand is slow and easy to get wrong.
A loop repeats the same idea for each index.

---

## 1) The `for` loop (best when you know how many times)

Use `for` when you already know the number of repeats: **the array length**, or "do this 5 times".

### Pattern to memorize

```java
for (int i = 0; i < names.length; i++) {
    System.out.println(names[i]);
}
```

The parentheses have **three parts**, separated by `;`:

| Part | Example | Meaning |
|------|---------|---------|
| start | `int i = 0` | create the counter; start at the first index |
| condition | `i < names.length` | keep looping **while this is true** |
| step | `i++` | after each turn, go to the next index (`i = i + 1`) |

Inside `{ }` you write what to do **one time per index**.

### Why `for` is good for arrays

- Start, stop, and step are **in one line** — hard to forget `i++`
- Perfect when you walk **every** element from 0 to length-1
- Other programmers expect `for` for this job

---

## 2) The `while` loop (best when you repeat until a condition changes)

`while` only asks: **"is this still true?"**
You must create the counter **before**, and move it **inside** the loop yourself.

### Pattern to memorize

```java
int i = 0;
while (i < names.length) {
    System.out.println(names[i]);
    i++;
}
```

Read it as: *while `i` is still a valid index, print, then add 1 to `i`*.

### Important

If you forget `i++` inside `while`, the condition stays true forever. That is an **infinite loop**. The program never ends.

`for` puts `i++` in the header, so this mistake is less common with arrays.

### Why `while` is good

- You do **not** always know the count in advance
- Example ideas (later in programming): keep asking until the password is correct; keep adding while the total is under 100
- The stop condition can be anything boolean, not only `i < length`

Small example (not an array — just the idea):

```java
int lives = 3;
while (lives > 0) {
    // play
    lives = lives - 1;
}
```

Here you loop **until lives is 0**, not "exactly 3 times" written as a fixed array walk. `while` matches that story better.

---

## 3) Same job, two styles (arrays)

These two programs do the **same** thing:

```java
// for — usual choice for arrays
for (int i = 0; i < nums.length; i++) {
    System.out.println(nums[i]);
}

// while — also correct, but you manage i yourself
int i = 0;
while (i < nums.length) {
    System.out.println(nums[i]);
    i++;
}
```

**Typical:** `for` for a whole array. **Typical:** `while` until a condition changes.

That is only a habit, not a law.

---

## 4) Either loop can do any job

This is important:

- It is **correct** to use `for` or `while` in **any** of these situations.
- With enough creativity, you can rewrite one as the other.
- "Typical" means *easier to read*, not *the only legal way*.

### Array walk with `while` (not the usual choice, still correct)

You already saw this above. It works. You just manage `i` yourself.

### Countdown with `for` (not the usual choice, still correct)

`while (lives > 0)` matches the story. But this `for` does the same thing:

```java
for (int lives = 3; lives > 0; lives = lives - 1) {
    System.out.println(lives);
}
```

Same result: 3, then 2, then 1. It is not "wrong". It is just less natural than `while` for "until lives run out".

### Search with `for` (also correct)

You can loop the whole array with `for` even after you already found the item.
You can also use `while` and stop early (`while (!found && i < names.length)`).
Both are valid. Stopping early is a style choice.

**Rule for class:** pick the loop that is easier to read. If your teacher asks for `for` or `while`, that is for practice, not because the other one is illegal.

---

## 5) Quick decision (typical, not mandatory)

| Situation | Prefer | Why |
|-----------|--------|-----|
| Visit every index of an array | `for` | start, condition, and `i++` stay together |
| You know the exact number of repeats | `for` | the header shows the plan |
| Repeat until something becomes false | `while` | the condition is the story |
| You might stop in the middle (search, then done) | either; `while` is natural | you can loop while `!found` |

Both need a **boolean** condition. Both can loop forever if the condition never becomes false.

---

## Where to work
- File: `Starter.java`

**Open `Starter.java` first.** At the top there are 4 **EXAMPLE** blocks that already compile and run:
1. `for` over an array
2. the **same** walk with `while`
3. `while` until lives is 0
4. the **same** countdown with `for` (not typical, still correct)

Run the file once before writing your TODOs so you see the examples print.

## Steps

1. Create String[] `cities` with 3 city names (practice).
2. Use a **`for`** loop to print each city (new — copy the *idea* from the `for` pattern, write it yourself if the skeleton is incomplete).
3. Create int[] `nums` with 4 even numbers.
4. Print each number with a **`while`** loop (new — you must write `int i`, `while`, print, and `i++`).
5. Create int `lives` as 3.
6. Use a **`while`** loop: while lives > 0, print lives, then subtract 1.
   This is the typical "until a condition changes" case.
7. Do the **same countdown again** with a **`for`** (not typical, but correct — see section 4).
   Use a different variable name, for example `livesFor`.

## Done when
- Cities print with `for`
- Numbers print with `while`
- Lives print 3, 2, 1 with `while` **and** again with `for`

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
cd student-tasks\task-24-for-loop-array
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
