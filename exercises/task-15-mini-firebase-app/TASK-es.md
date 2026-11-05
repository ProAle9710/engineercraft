# Tarea 15 — Mini app Firebase (juntar todo)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Combinar variables Java + lógica simple + ideas iniciales de Firebase.

## Regla para guardar
Una nota se puede guardar solo si:
- `isLoggedIn` es `true` **y**
- (en esta tarea) usamos `canSave` a partir de `isLoggedIn`

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

### A) Config Firebase (demo)
1. Crea String `projectId` con class-demo.
2. Crea boolean `isDemoMode` en true.

### B) Usuario
3. Crea String `userName`.
4. Crea boolean `isLoggedIn`.

### C) Documento nota
5. Crea String `noteId`.
6. Crea String `noteText`.
7. Crea String `notePath` concatenando notes/ y noteId.

### D) Lógica
8. Crea boolean `canSave` a partir de `isLoggedIn`.
9. Asigna String `saveMessage` con if/else:
   - Si puede guardar → Note ready to save to Firebase
   - Si no → Cannot save yet

## Listo cuando
Todas las variables usen tipos Java correctos y la lógica cumpla la regla.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-15-mini-firebase-app
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
