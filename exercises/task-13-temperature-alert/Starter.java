// ============================================
// Task 13 — Temperature Alert Logic
// ============================================

public class Starter {
    public static void main(String[] args) {

        // TODO 1: Create String sensorId


        // TODO 2: Create int temperature


        // TODO 3: Create String unit


        // TODO 4: Complete the alert logic
        String alertMessage = "";
        if (temperature > 30) {
            // Hot

        } else if (temperature < 10) {
            // Cold

        } else {
            // Normal

        }

        // TODO 5: Create boolean needsAttention using || (hot OR cold)


        // -----------------------------------------
        // Do not change the lines below
        // -----------------------------------------
        System.out.println("Sensor: " + sensorId);
        System.out.println("Temperature: " + temperature + " " + unit);
        System.out.println("Alert: " + alertMessage);
        System.out.println("Needs attention: " + needsAttention);
    }
}
