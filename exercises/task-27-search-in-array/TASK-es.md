# Tarea 27 — Buscar un valor en un arreglo

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Comprobar si un nombre existe en un `String[]` con un ciclo y `.equals`.

## Lógica

```
found empieza en false
por cada nombre:
    si ese nombre equals al objetivo:
        found = true
```

`names[i]` es un String, así que puedes usar el **punto**: `names[i].equals(target)`

## Elección de ciclo

Típico para buscar: `while` (`!found && i < names.length`) para poder parar al encontrar.
Un `for` que recorre todo también es **correcto**.
Esta tarea: busca con **`while`**, imprime todos los nombres con **`for`**.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `names` con 4 nombres.
2. Crea String `target` (luego prueba un nombre que no esté).
3. Crea boolean `found` empezando en false.
4. Busca con un **`while`** usando `.equals`.
5. Imprime cada nombre con un **`for`**.
6. Asigna String `message` con if/else: User found / User not found.

## Listo cuando
`"Maria"` da true, y un nombre que no está da false.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-27-search-in-array
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
