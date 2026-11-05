# Tarea 42 — Reglas de nombre de usuario

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Comprobar varias reglas sobre un String y decidir con if/else (varias acciones).

## Reglas
- La longitud debe ser al menos 5.
- El primer carácter no debe ser un espacio.
- No debe ser igual a admin (usa `.equals`).

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `username`.
2. Crea int `size` usando `.length()`.
3. Crea char `first` usando `.charAt(0)` (si size es 0 no puedes llamar charAt — usa if/else y pon `'?'` si está vacío).
4. Crea boolean `longEnough`.
5. Crea boolean `noSpace` (first no es `' '`).
6. Crea boolean `notAdmin` (no es igual a admin).
7. Crea boolean `valid` usando `&&` con los tres boolean.
8. Usa if/else:
   - Si valid: mensaje OK y level 1
   - Si no: mensaje REJECTED y level 0
9. Crea un String largo `log` (imagina que este texto iría a Firebase después).

## Listo cuando
Pruebes tres usernames: uno válido, admin, y uno corto.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-42-username-rules
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
