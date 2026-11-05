# Tarea 35 — Imprimir un arreglo al revés

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Recorrer desde el último índice hasta 0.

## Idea del ciclo

Empieza `i` en `items.length - 1`.
Sigue mientras `i >= 0`.
Después de cada vuelta, `i--` (menos uno).

Ya conoces `i++`. `i--` es la misma idea, hacia atrás.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `steps` con 4 textos cortos.
2. Imprímelos en orden normal con un **`for`** (típico).
3. Imprímelos al revés con un **`while`** hacia atrás (`i--`). También sería correcto con `for`; aquí debes usar `while`.
4. Crea String `lastStep` del último índice.
5. Crea String `firstStep` del índice 0.

## Listo cuando
El orden inverso muestre el último ítem primero.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-35-print-reverse
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
