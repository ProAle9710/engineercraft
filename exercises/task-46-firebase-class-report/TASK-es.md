# Tarea 46 — Mini reporte de grupo (juntar el nivel 3)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Armar un pequeño reporte "se guardaría en Firebase" con lo que ya sabes:
arreglos, ciclos, `.equals`, concatenación, boolean, ternario, if/else.

## Historia
Tienes usuarios en la colección `grades`. Cada uno tiene nombre y nota.
Vas a buscar un usuario, calcular estadísticas del grupo y armar un texto de documento largo.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `collectionName` (grades).
2. Crea String[] `ids`, String[] `names`, int[] `scores` (longitud 4, mismo orden).
3. Crea String `searchId`.
4. Busca el id con un **`while`**. Conserva nombre y nota, o found = false. Un `for` también sería correcto.
5. Crea String `docPath` con un ternario (colección/id o NOT_FOUND).
6. Calcula promedio y passCount con un **`for`**.
7. Crea boolean `canPublish` (found Y average >= 60).
8. Si canPublish, saveMessage listo para guardar; si no, explica qué falta (if/else con más de una frase está bien).
9. Crea String `document` como un texto largo: path, resultado de búsqueda, average, passCount, saveMessage.

## Listo cuando
Cambiar searchId y scores cambie path, found y canPublish de forma coherente.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-46-firebase-class-report
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
