# Tarea 39 — Corregir notas demasiado altas

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Recorrer un int[] y cambiar cualquier valor mayor que 100 a 100.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea int[] `scores` que incluya valores por encima de 100 y algunos por debajo de 60.
2. Usa un **`for`** y limita los valores por encima de 100.
3. Crea int `passCount` empezando en 0.
4. Usa un **`while`** para contar cuántos scores son >= 60.
5. Crea int `failCount` con length menos passCount.
6. Crea String `summary` concatenando.
7. Crea String `light` con un ternario (GREEN si passCount >= failCount, si no RED).

## Listo cuando
Ningún score impreso sea mayor que 100, y los conteos coincidan con los valores ya limitados.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-39-cap-scores
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
