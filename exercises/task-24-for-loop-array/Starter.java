// ============================================
// Task 24 — Loops: for and while
// READ the EXAMPLE blocks first. They already run.
// Then complete YOUR TURN (TODO).
//
// for: typical when you know how many times (arrays)
// while: typical until a condition changes
// Either loop can do any job. Typical = easier to read, not the only legal way.
// ============================================

public class Starter {
    public static void main(String[] args) {

        // ----------------------------------------------------------
        // EXAMPLE 1 — for over an array (typical)
        // Three parts: start ; condition ; step
        //   int i = 0           -> start at index 0
        //   i < demoNames.length -> keep going while i is valid
        //   i++                 -> next index after each turn
        // ----------------------------------------------------------
        String[] demoNames = {"Ana", "Luis", "Maria"};
        System.out.println("EXAMPLE 1 — for (every name):");
        for (int i = 0; i < demoNames.length; i++) {
            System.out.println("  index " + i + " = " + demoNames[i]);
        }

        // ----------------------------------------------------------
        // EXAMPLE 2 — while doing THE SAME array walk
        // Not the usual choice for a full array, but 100% correct.
        // You create i BEFORE the loop.
        // You MUST write i++ INSIDE, or the loop never ends.
        // ----------------------------------------------------------
        System.out.println("EXAMPLE 2 — while (same names, also correct):");
        int n = 0;
        while (n < demoNames.length) {
            System.out.println("  index " + n + " = " + demoNames[n]);
            n++;
        }

        // ----------------------------------------------------------
        // EXAMPLE 3 — while until a condition changes (typical)
        // Story: keep going WHILE lives > 0.
        // ----------------------------------------------------------
        System.out.println("EXAMPLE 3 — while (lives until 0):");
        int demoLives = 3;
        while (demoLives > 0) {
            System.out.println("  lives = " + demoLives);
            demoLives = demoLives - 1;
        }

        // ----------------------------------------------------------
        // EXAMPLE 4 — SAME countdown with for (not typical, still correct)
        // Creativity: for can also stop on a condition, not only arrays.
        // ----------------------------------------------------------
        System.out.println("EXAMPLE 4 — for (same countdown, also correct):");
        for (int life = 3; life > 0; life = life - 1) {
            System.out.println("  lives = " + life);
        }

        // ----------------------------------------------------------
        // YOUR TURN — write YOUR loops. Do not only copy the examples.
        // Use new variable names: cities, nums, lives, livesFor
        // ----------------------------------------------------------

        // TODO 1: Create String[] cities with 3 city names


        System.out.println("=== Cities (your for) ===");
        // TODO 2: Print each city with a for loop
        // Look at EXAMPLE 1. Write the for yourself.


        // TODO 3: Create int[] nums with 4 numbers


        System.out.println("=== Numbers (your while) ===");
        // TODO 4: Print each number with a while loop
        // Look at EXAMPLE 2: int i = 0; while (...); i++;


        // TODO 5: Create int lives as 3


        System.out.println("=== Lives (your while until 0) ===");
        // TODO 6: while lives > 0: print lives, then subtract 1
        // Look at EXAMPLE 3.


        System.out.println("=== Lives again (your for — not typical, still correct) ===");
        // TODO 7: Same countdown with a for loop
        // Look at EXAMPLE 4. Use a new name, for example livesFor.

    }
}
