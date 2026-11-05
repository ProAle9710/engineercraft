# Tarea 43 — Promedio del grupo y estado

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Sumar, promedio, contar aprobados y armar un reporte. Todo en un programa.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `names` con 5 estudiantes.
2. Crea int[] `scores` con 5 notas.
3. Crea int `total` y suma las notas con un **`while`**.
4. Crea int `average` (total / length).
5. Crea int `passCount`.
6. Usa un **`for`**: imprime name + score + un ternario Pass/Fail en esa fila.
7. Crea boolean `classOk` (average >= 70 Y passCount >= 3).
8. Crea String `banner` con un ternario.
9. Crea String `report` que incluya average, passCount, banner.

## Listo cuando
Cada fila se imprima, y classOk cumpla las dos reglas a la vez.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-43-class-average
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
