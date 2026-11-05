# Tarea 12 — Carrito con ternario + String largo

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Calcular un total, elegir un valor con un **ternario**, y armar un recibo largo.

## Ternario para un número

```java
int finalTotal = hasDiscount ? subtotal - 2 : subtotal;
```

Misma idea que if/else, pero en una línea, y **devuelve un valor**.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `productName`.
2. Crea int `unitPrice`.
3. Crea int `quantity`.
4. Crea int `subtotal` (precio por cantidad).
5. Crea boolean `hasDiscount`.
6. Crea int `finalTotal` con un **ternario** (descuento de 2 si hasDiscount, si no subtotal).
   El ejemplo de arriba es la idea nueva — escríbelo tú.
7. Crea String `discountLabel` con un ternario: YES o NO.
8. Crea String `receipt` concatenando varias piezas (usa `\n` para nueva línea).
   Incluye producto, unit, qty, subtotal, discount, total.

## Consejo
`\n` significa salto de línea dentro de un String.

## Listo cuando
`finalTotal` y `discountLabel` usen ternarios, y `receipt` sea un String largo.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-12-shopping-cart-vars
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
