# Tarea 36 — El menor y el mayor número

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Recorrer un int[] y guardar el valor más pequeño y el más grande.

## Idea
Empieza `min` y `max` como `numbers[0]`.
Luego recorre desde el índice 1.
Si el actual es más pequeño, actualiza `min`.
Si es más grande, actualiza `max`.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea int[] `numbers` con al menos 5 valores (mezcla altos y bajos).
2. Crea int `min` e int `max` desde el índice 0.
3. Usa un **`for`** para actualizar min / max (típico para cada número).
3b. Imprime cada número con un **`while`**.
4. Crea int `spread` como max menos min.
5. Crea String `summary` concatenando.
6. Crea String `alert` con un ternario (spread mayor que 50 o no).

## Listo cuando
Min, max y spread coincidan con una comprobación a mano.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-36-min-and-max
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
