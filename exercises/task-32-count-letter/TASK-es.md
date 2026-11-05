# Tarea 32 — Contar una letra en una palabra

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Usar `toCharArray()`, un ciclo e `if` para contar cuántas veces aparece una letra.

## Recuerdo (char)

```java
char letter = 'a';
if (letters[i] == letter) { ... }
```

Para `char` usa `==` (no `.equals`). Comillas simples: `'a'`.

## Elección de ciclo

Típico para cada carácter: `for`. `while` también es correcto.
Esta tarea: **`for`** para contar, **`while`** para imprimir cada char.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `word` con al menos 6 letras.
2. Crea char `target` (la letra a contar).
3. Crea `char[] letters` a partir de la palabra.
4. Crea int `count` empezando en 0.
5. Usa un **`for`**. Si el char actual es igual a `target`, suma 1.
6. Usa un **`while`** para imprimir cada carácter.
7. Crea String `report` concatenando palabra, letra y conteo.
8. Crea String `status` con un ternario: found vs not found (`count > 0`).

## Listo cuando
Si la palabra es banana y la letra es a, count es 3.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-32-count-letter
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
