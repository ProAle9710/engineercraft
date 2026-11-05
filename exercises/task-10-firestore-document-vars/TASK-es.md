# Tarea 10 — Un documento Firestore como variables

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Representar un documento de Firestore con variables Java.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

Escribe las variables tú. Concatena con `+` como en la Tarea 05.

1. Crea String `collectionName` con el texto books.
2. Crea String `documentId` con el texto book_1.
3. Crea String `title`.
4. Crea String `author`.
5. Crea int `pages`.
6. Crea boolean `available`.
7. Crea String `path` uniendo collectionName, `/` y documentId.
   Idea esperada: books/book_1

## Listo cuando
Todos los campos tengan tipos correctos y `path` se vea como books/book_1.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-10-firestore-document-vars
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
