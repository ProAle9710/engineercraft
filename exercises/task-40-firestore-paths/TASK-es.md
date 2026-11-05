# Tarea 40 — Armar muchos paths de Firestore

Si prefieres inglés, abre `TASK.md`.

## Objetivo
A partir de un arreglo de ids, armar paths como `users/u1`.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `collectionName` con users.
2. Crea String[] `ids` con 4 ids.
3. Crea String[] `paths` con `new String[ids.length]`.
4. Llena cada `paths[i]` con un **`for`** (concatenación).
5. Crea String `wantedId`.
6. Busca con un **`while`** y asigna boolean `exists` (típico para buscar; `for` también es correcto).
7. Crea String `wantedPath` con un ternario: si existe, arma el path; si no, NOT_FOUND.
8. Imprime todos los paths con **`for`** o **`while`** (tú eliges — ambos correctos), luego wantedPath.

## Listo cuando
Se impriman cuatro paths, y wantedPath sea correcto tanto para un id real como para uno que no esté.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-40-firestore-paths
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
