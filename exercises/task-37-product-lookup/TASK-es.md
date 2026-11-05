# Tarea 37 — Buscar un producto y su precio

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Buscar en un String[] y, al encontrar el nombre, leer el precio en el mismo índice de un int[].

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `products` con 4 nombres.
2. Crea int[] `prices` con 4 precios (mismo orden).
3. Crea String `wanted` (uno de los nombres; luego uno que no esté).
4. Crea boolean `found` en false.
5. Crea int `priceFound` en 0.
6. Busca con un **`while`** (típico cuando puedes parar al encontrar). Un `for` también sería correcto.
6b. Imprime el catálogo completo con un **`for`**.
7. Crea String `result` con if/else (found vs not found). Incluye el precio si lo encontraste.
8. Crea String `tag` con un ternario: IN STOCK vs MISSING.

## Listo cuando
Un producto real muestre su precio. Un nombre que no esté muestre not found.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-37-product-lookup
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
