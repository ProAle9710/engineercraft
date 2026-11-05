# Tarea 28 — Arreglos paralelos (nombres + notas)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Usar dos arreglos juntos: el mismo índice = la misma persona.

## Idea

```
índice:    0       1       2
nombres: "Ana"   "Luis"  "Maria"
notas:     90      70      85
```

Así `names[1]` y `scores[1]` pertenecen a Luis.

## Elección de ciclo

Típico para imprimir cada par: `for`.
Encontrar el mejor puede ser `for` o `while` — ambos correctos.
Esta tarea: **`for`** para imprimir, **`while`** para el puntaje más alto.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `names` con 3 nombres.
2. Crea int[] `scores` con 3 notas (mismo orden).
3. Imprime cada estudiante con un **`for`**.
4. Empieza con la primera persona como el mejor actual (nombre + nota).
5. Encuentra la nota más alta con un **`while`**.

## Listo cuando
Se imprimen los pares y el mejor es coherente con tus números.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-28-parallel-arrays
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
