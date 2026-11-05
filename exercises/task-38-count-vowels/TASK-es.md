# Tarea 38 — Contar vocales

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Convertir un String en `char[]` y contar vocales: a e i o u (solo minúsculas en esta tarea).

## Consejo
Puedes usar varios `||` en un if:

```java
if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
```

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `text` (usa minúsculas).
2. Crea `char[] letters` a partir del texto.
3. Crea int `vowels` empezando en 0.
4. Crea int `other` empezando en 0.
5. Usa un **`for`** para contar vocales vs other.
5b. Imprime cada carácter con un **`while`**.
6. Crea boolean `moreVowels` (vowels mayor que other).
7. Crea String `report` concatenando.

## Listo cuando
Para "java" debes obtener 2 vocales (a, a) y 2 otras (j, v).

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-38-count-vowels
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
