# Tarea 26 — Contar + ternario (if corto)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
1) Contar notas aprobadas con un ciclo.
2) Aprender el **operador ternario**: un if/else corto que devuelve un valor.

## Ternario = otra forma de if/else

```java
String label = didPass ? "Pass" : "Fail";
```

Léelo: `condicion ? valorSiTrue : valorSiFalse`

## Elección de ciclo

Típico: `for` para visitar cada score. `while` también es correcto.
Esta tarea: **`for`** para contar aprobados, **`while`** para imprimir cada score.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea int[] `scores` con 55, 80, 42, 90, 60.
2. Crea int `passCount` empezando en 0.
3. Usa un **`for`**: si un score es al menos 60, suma 1 a `passCount`.
4. Usa un **`while`** para imprimir cada score.
5. Crea int `failCount` con length menos passCount.
6. Crea boolean `mostlyPassing`.
7. Crea String `summary` con un ternario.
8. Crea String `firstResult` con un ternario para scores[0] (Pass / Fail).

## Listo cuando
`passCount` es `3`, `failCount` es `2`, y ambos Strings ternarios existen.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-26-count-with-if
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
