# Tarea 41 — Quiz: ¿cuántas respuestas son correctas?

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Dos String[] de la misma longitud: `answers` (estudiante) y `key` (correctas).
Cuenta coincidencias con `.equals`.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `key` con 5 respuestas cortas (ejemplo: A, B, C, A, D).
2. Crea String[] `answers` con 5 respuestas del estudiante (algunas mal).
3. Crea int `correct` empezando en 0.
4. Usa un **`for`**: si answers[i] equals key[i], suma 1.
4b. Imprime cada par (key vs answer) con un **`while`**.
5. Crea int `wrong` con length menos correct.
6. Crea int `percent` como correct * 100 / key.length (división entera está bien).
7. Crea String `grade` con if / else if / else:
   - percent >= 80 → A
   - percent >= 60 → B
   - si no → F
8. Crea un String largo `report`.

## Listo cuando
correct + wrong sea 5, y grade coincida con la regla del percent.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-41-quiz-matches
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
