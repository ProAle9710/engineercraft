# Tarea 44 — Copiar aprobados a un arreglo nuevo

Si prefieres inglés, abre `TASK.md`.

## Objetivo
1) Contar cuántos aprueban.
2) Crear un String[] de ese tamaño exacto.
3) Copiar solo los nombres aprobados.

## Patrón de dos ciclos

Ciclo 1: contar.
Luego `String[] passed = new String[count];`
Ciclo 2: copiar. Necesitas un segundo índice `p` que solo crece cuando copias.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `names` e int[] `scores` (misma longitud, al menos 5).
2. Crea int `count` de scores >= 60 usando un **`for`**.
3. Crea String[] `passed` con `new String[count]`.
4. Crea int `p` empezando en 0.
5. El segundo ciclo debe ser **`while`**: si el estudiante aprueba, pon el nombre en `passed[p]` y suma 1 a `p`. (`for` también funcionaría.)
6. Imprime el arreglo passed.
7. Crea String `joined` concatenando esos nombres con espacios.
8. Crea String `note` con un ternario (alguien aprobó vs nadie).

## Listo cuando
`passed.length` sea igual al número de aprobados y los nombres coincidan.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-44-copy-passing-names
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
