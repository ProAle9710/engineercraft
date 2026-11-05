# Tarea 34 — Unir nombres en un String largo

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Recorrer un arreglo y armar un String con comas: Ana, Luis, Maria

## Truco
No pongas coma después del último nombre.

Idea: si este no es el último índice, agrega `", "` después del nombre.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `names` con 4 nombres.
2. Crea String `list` vacío.
3. Usa un **`for`** para armar `list` con comas (típico para cada nombre).
3b. Imprime cada nombre con un **`while`** (también correcto).
4. Crea int `howMany` con el length del arreglo.
5. Crea String `header` concatenando (cuántas personas).
6. Crea String `full` = header + línea nueva + list.

## Listo cuando
La lista impresa tenga comas entre nombres, pero no después del último.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-34-join-names
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
