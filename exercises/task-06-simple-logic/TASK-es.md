# Tarea 06 — Lógica simple (if / else)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Usar un boolean para elegir qué mensaje mostrar.

## Idea

```java
if (isRaining) {
    advice = "Take an umbrella";
} else {
    advice = "Enjoy the sun";
}
```

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea boolean `isRaining`.
2. Crea String `advice` (empieza vacío).
3. Dentro del `if / else`, asigna `advice` correctamente.

## Listo cuando
- Si `isRaining` es `true`, advice es `"Take an umbrella"`.
- Si `isRaining` es `false`, advice es `"Enjoy the sun"`.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-06-simple-logic
```

### 3) Compila

```bash
javac Starter.java
```

### 4) Ejecuta

```bash
java Starter
```

- Correcto: `java Starter`
- Incorrecto: `java Starter.java`
