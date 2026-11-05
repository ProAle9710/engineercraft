# Tarea 13 — Alerta de temperatura

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Practicar lógica con temperaturas.

## Reglas
- Si la temperatura es mayor que `30` → alerta `"Hot"`
- Si no, si es menor que `10` → alerta `"Cold"`
- Si no → alerta `"Normal"`

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `sensorId`.
2. Crea int `temperature`.
3. Crea String `unit` con C.
4. Completa `if / else if / else` para `alertMessage` (Hot / Cold / Normal).
5. Crea boolean `needsAttention` usando OR (`||`): true si está caliente o frío.

## Listo cuando
Pruebes temperaturas como `35`, `5` y `22`.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-13-temperature-alert
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
