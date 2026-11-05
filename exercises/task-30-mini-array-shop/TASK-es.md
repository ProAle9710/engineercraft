# Tarea 30 — Strings largos + ternario

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Practicar **concatenar** Strings (`+`) para armar un texto largo,
y usar un **ternario** para el mensaje final.

## Concatenación

```java
String line = "Product: " + "Pen" + " - price: " + 5;
```

También puedes ir creciendo un String:

```java
String report = "";
report = report + "Line 1\n";
```

(`\n` es una línea nueva.)

## Recordatorio de ternario

```java
String msg = canBuyAll ? "You can buy everything" : "Not enough money";
```

## Elección de ciclo

Típico para el catálogo: `for`.
Sumar con `while` también es correcto.
Esta tarea usa **los dos**.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `products` con 3 nombres.
2. Crea int[] `prices` con 3 precios (mismo orden).
3. Crea String `catalog` vacío.
4. Usa un **`for`** para ir concatenando el catálogo.
5. Crea int `total` en 0 y suma los precios con un **`while`**.
6. Crea boolean `canBuyAll` (total a lo sumo 50).
7. Crea String `checkoutMessage` con un ternario.
8. Crea un String largo `fullReport` (catálogo + total + mensaje).

## Listo cuando
`fullReport` sea un String largo. Con los precios 5, 20, 3 el total es `28`.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-30-mini-array-shop
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
