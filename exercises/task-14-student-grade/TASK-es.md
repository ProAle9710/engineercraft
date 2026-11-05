# Tarea 14 — Nota del estudiante con ternario

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Decidir aprobado / reprobado con un **boolean** y un **ternario** (`? :`).
También armar un reporte largo concatenando.

## Ternario (if/else corto que da un valor)

```java
String status = didPass ? "Pass" : "Fail";
```

Léelo: `condicion ? valorSiTrue : valorSiFalse`

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

Práctica. **No** copies una línea terminada — escríbela.

1. Crea String `studentName`.
2. Crea String `courseName`.
3. Crea int `score` (0 a 100).
4. Crea int `passingScore` en 60.
5. Crea boolean `didPass` usando `>=`.
6. Crea String `status` con un ternario: Pass o Fail.
7. Crea String `light` con un ternario: GREEN o RED.
8. Crea un String largo `report` concatenando nombre, curso, score, status, light.
9. Crea String `firebaseCollection` con el texto grades.

## Listo cuando
`status` y `light` salgan de ternarios, y `report` sea un String largo.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-14-student-grade
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
