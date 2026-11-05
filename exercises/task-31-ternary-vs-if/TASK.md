# Task 31 — Ternary vs if/else (Differences and Limits)

## Goal
Understand **when to use** a ternary (`? :`) and **when to use** `if / else`.
They are related, but they are not the same tool.

## 1) Same idea, different shape

### if / else (statements — do actions)

```java
String status;
if (didPass) {
    status = "Pass";
} else {
    status = "Fail";
}
```

### Ternary (expression — gives a value)

```java
String status = didPass ? "Pass" : "Fail";
```

Both need a **boolean** condition (`true` / `false`).

## 2) What a ternary is good for

Use a ternary when:

- You choose **one value** between two options
- The code stays **short and clear**
- You assign that value to a variable (or return it)

Examples:

```java
int fee = isMember ? 0 : 10;
String label = isOnline ? "Online" : "Offline";
```

## 3) Limits of the ternary (important)

A ternary is **NOT** good when:

1. You need **many lines** of work in each branch  
   (calling several prints, changing many variables, etc.)
2. You have **more than two clear options** (use `if / else if / else`)
3. The line becomes hard to read (too long or nested ternaries)

Bad beginner habit:

```java
// Hard to read — avoid nested ternaries for now
String x = a ? (b ? "1" : "2") : "3";
```

Use normal `if / else` instead.

## 4) Limits of if / else

`if / else` is more flexible, but:

- It is longer for a simple choice of one value
- It is a **statement**: it does not by itself "become" a value in one expression
- For a tiny choice like `"Pass"` / `"Fail"`, a ternary is often clearer

## 5) Quick decision table

| Situation | Prefer |
|-----------|--------|
| Choose one simple value | Ternary `? :` |
| Several actions in a branch | `if / else` |
| 3 or more paths | `if / else if / else` |
| Code is getting confusing | Prefer `if / else` |

## Where to work
- File: `Starter.java`

## Steps

The examples above teach the idea. In `Starter.java` write the code yourself.

1. Create boolean `isLoggedIn` (also try false later).
2. Create boolean `hasProfile`.
3. Create String `gate` with a ternary (Enter vs Login required).
4. Create int `bonus` with a ternary (5 vs 0).
5. Use **if / else** (not ternary) because you change TWO variables:
   - If hasProfile: Profile OK and ready true
   - Else: Create profile and ready false
6. Create a long String `summary` with concatenation.
7. Create String `finalLine` with a ternary (GO vs WAIT).

## Done when
You can explain in your own words:
- Ternary = choose a **value**
- if/else = choose **actions / paths**
- Do not nest ternaries yet
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
cd student-tasks\task-31-ternary-vs-if
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
