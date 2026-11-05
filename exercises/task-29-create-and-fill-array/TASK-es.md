# Tarea 29 — Crear un arreglo vacío y llenarlo

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Crear un arreglo con tamaño fijo y luego poner valores en cada índice.

## Dos formas de crear arreglos

Ya conocida:

```java
int[] a = {1, 2, 3};
```

Nueva (tamaño primero, valores después):

```java
int[] a = new int[3];
a[0] = 10;
a[1] = 20;
a[2] = 30;
```

Para Strings:

```java
String[] ids = new String[3];
ids[0] = "user_1";
```

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String[] `userIds` de tamaño 3 con `new` (sintaxis nueva).
2. Llena las 3 casillas con user_1, user_2, user_3.
3. Crea int[] `levels` de tamaño 3 con `new`.
4. Llena levels con 1, 2, 3.
5. Imprime todos los userIds con un **`for`** (incluye el level).
6. Imprímelos otra vez con un **`while`** (también correcto).

## Listo cuando
Todas las casillas estén llenas e impresas de las dos formas.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-29-create-and-fill-array
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
