# Tarea 25 — Sumar los números de un arreglo

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Usar un ciclo para sumar todos los valores de un `int[]`.

## Lógica

```
total empieza en 0
suma scores[0]
suma scores[1]
...
```

En código (ejemplo de enseñanza):

```java
int total = 0;
for (int i = 0; i < scores.length; i++) {
    total = total + scores[i];
}
```

## Elección de ciclo

Típico para sumar todo el arreglo: `for`.
Un `while` con `i` e `i++` también es **correcto**.
En esta tarea usa **los dos**: `for` para sumar, `while` para imprimir cada score.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea int[] `scores` con 10, 20, 30, 40.
2. Crea int `total` empezando en 0.
3. Usa un **`for`** para que `total` sea la suma (típico para arreglos).
4. Usa un **`while`** para imprimir cada score (también correcto).
5. Crea int `average` como total dividido entre cuántos scores hay.

## Listo cuando
Para `{10, 20, 30, 40}`, total es `100` y average es `25`.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-25-sum-array
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
