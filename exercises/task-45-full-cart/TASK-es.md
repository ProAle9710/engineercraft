# Tarea 45 — Carrito completo (cantidad x precio)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Arreglos paralelos: productos, precios, cantidades.
Calcular totales de línea, total del carrito, descuento y un recibo largo.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `products` con 3 nombres.
2. Crea int[] `prices` con 3 precios unitarios.
3. Crea int[] `qty` con 3 cantidades.
4. Crea int[] `lineTotal` con `new int[3]`.
5. Llena `lineTotal[i]` con un **`for`** (precio * qty).
6. Crea int `subtotal` como la suma de lineTotal.
7. Crea boolean `hasCoupon`.
8. Crea int `finalTotal` con un ternario (si hay cupón, subtotal - 5, pero nunca debajo de 0 — si hace falta dos pasos, usa if/else).
9. Crea String `receipt` creciéndolo en un **`while`**, luego agrega subtotal y finalTotal.

## Listo cuando
Cada total de línea sea correcto y el recibo sea un solo String largo.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-45-full-cart
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
