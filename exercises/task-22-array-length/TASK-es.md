# Tarea 22 — array.length (¿cuántos elementos?)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Preguntarle a un arreglo cuántos elementos tiene.

## Diferencia importante

| Código | Se usa en | Significado |
|--------|-----------|-------------|
| `text.length()` | String | método con `()` |
| `items.length` | arreglo | propiedad **sin** `()` |

```java
String[] items = {"A", "B", "C"};
int count = items.length; // 3
```

Fíjate: **sin paréntesis** después de `length` en arreglos.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `users` con 4 valores (práctica).
2. Crea int `userCount` usando `.length` del arreglo (nuevo — sin `()`).
3. Crea int[] `grades` con 3 valores.
4. Crea int `gradeCount` usando `.length`.
5. Crea boolean `hasThreeGrades` (¿gradeCount es igual a 3?).

## Listo cuando
Hayas usado `.length` (sin `()`) en arreglos y el programa corra.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-22-array-length
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
