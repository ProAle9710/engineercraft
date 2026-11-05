// ============================================
// Task 15 — Mini Firebase App
// Correct syntax: String x = ""; int n = 0; boolean b = true;
// ============================================

public class Starter {
    public static void main(String[] args) {

        // --- A) Firebase config (demo) ---

        // TODO 1: Create String projectId


        // TODO 2: Create boolean isDemoMode


        // --- B) User ---

        // TODO 3: Create String userName


        // TODO 4: Create boolean isLoggedIn


        // --- C) Note document ---

        // TODO 5: Create String noteId


        // TODO 6: Create String noteText


        // TODO 7: Create String notePath with concatenation


        // --- D) Logic ---

        // TODO 8: Create boolean canSave from isLoggedIn


        // TODO 9: Set saveMessage using if / else
        String saveMessage = "";
        if (canSave) {
            // ready to save

        } else {
            // cannot save yet

        }

        // -----------------------------------------
        // Do not change the lines below
        // -----------------------------------------
        System.out.println("=== Mini Firebase App Preview ===");
        System.out.println("Project: " + projectId + " | Demo: " + isDemoMode);
        System.out.println("User: " + userName + " | Logged in: " + isLoggedIn);
        System.out.println("Note path: " + notePath);
        System.out.println("Note text: " + noteText);
        System.out.println("Can save: " + canSave);
        System.out.println("Message: " + saveMessage);
    }
}
