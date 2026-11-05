# Tarea 33 — Primer y último carácter

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Combinar `length()`, `charAt`, boolean y un ternario.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `code` (idea: un código corto de producto).
2. Crea int `size` con la longitud del String.
3. Crea boolean `tooShort` (true si size es menor que 2).
4. Si `tooShort` es true, deja char `first` y char `last` en `'?'`.
   Si no, lee el primer y el último carácter reales.
   Usa if/else (dos asignaciones — no es buen ternario).
5. Crea boolean `sameEnds` (first igual a last). Usa `==` para char.
6. Crea String `message` con un ternario.

## Listo cuando
Pruebes códigos como ABBA (mismos extremos) y JAVA (distintos). También un String de 1 letra.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-33-first-last-char
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
