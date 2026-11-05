# Tarea 11 — Lógica de estado de login

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Practicar el estado de login con boolean y `&&` (Y).

## AND lógico

```java
boolean canEnter = isLoggedIn && hasEmail;
```

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea boolean `isLoggedIn`.
2. Crea boolean `hasEmail`.
3. Crea String `userEmail` (texto vacío si hasEmail es false).
4. Crea boolean `canEnterHome` usando AND (`&&`) con los dos boolean de arriba.
5. Asigna String `screenMessage` con if/else:
   - Si puede entrar → Welcome to Home
   - Si no → Please log in

## Listo cuando
Pruebes combinaciones true/false y el mensaje coincida.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-11-auth-status-logic
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
