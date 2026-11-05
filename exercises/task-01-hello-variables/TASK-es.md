# Tarea 01 — Hola variables

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Crear tus primeras variables **String** (texto) con sintaxis correcta de Java.

## ¿Qué es un String?
Un String es texto. En Java debes escribir el tipo `String` antes del nombre:

```java
String city = "Havana";
```

## Sintaxis correcta

```java
String firstName = "Luis";
```

Incorrecto (no uses esto):

```java
let firstName = "Luis";   // JavaScript — no es de esta clase
var firstName = "Luis";   // no es el estilo de aquí
```

## Dónde trabajar
Escribe el código en:

- `Starter.java`

Busca comentarios que empiezan con `TODO`.
Escribe las variables **dentro** del método `main`.

## Pasos

1. Crea `String firstName = "...";` con tu nombre.
2. Crea `String lastName = "...";` con tu apellido.
3. Crea `String favoriteColor = "...";` con un color.
4. **No** borres las líneas de `System.out.println` de abajo.

## Listo cuando
- Tengas 3 variables String con `String name = "...";`
- El programa imprima tus valores (sin errores).

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta de la tarea

Debes estar en la carpeta que contiene `Starter.java`.

```bash
cd student-tasks\task-01-hello-variables
```

Comprueba con `dir`. Debes ver `Starter.java`, `TASK.md` y `TASK-es.md`.

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
