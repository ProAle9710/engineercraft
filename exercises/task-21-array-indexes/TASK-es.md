# Tarea 21 — Índices de un arreglo (posiciones)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Leer un valor de un arreglo usando su **índice** (posición).

## Los índices empiezan en 0

```java
String[] fruits = {"Apple", "Banana", "Cherry"};
// índice:             0         1         2
```

| Código | Resultado |
|--------|-----------|
| `fruits[0]` | `"Apple"` |
| `fruits[1]` | `"Banana"` |
| `fruits[2]` | `"Cherry"` |

Error típico: creer que el primero es `[1]`. En Java es `[0]`.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `days` con 5 nombres cortos de día (práctica de la Tarea 19).
2. Crea String `firstDay` del índice 0 (nuevo — mira la tabla).
3. Crea String `thirdDay` del índice 2.
4. Crea String `lastDay` del último índice.
5. Crea int[] `points` con 3 números (práctica de la Tarea 20).
6. Crea int `middle` del índice 1 de `points`.

## Listo cuando
Puedas decir en voz alta: "el índice 0 es el primer elemento".

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-21-array-indexes
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
