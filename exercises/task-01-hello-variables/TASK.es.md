# Tarea 01 — Hola variables

> Español: este archivo. Inglés: `TASK.md`

## Objetivo
Crear tus primeras variables **String** (texto) con sintaxis Java correcta.

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
- Archivo: `Starter.java`
- Escribe dentro del método `main`.
- Busca comentarios `TODO`.

## Pasos

1. Crea `String firstName = "...";` con tu nombre.
2. Crea `String lastName = "...";` con tu apellido.
3. Crea `String favoriteColor = "...";` con un color.
4. No borres las líneas `System.out.println` del final.

## Listo cuando
- Tienes 3 variables String con `String nombre = "...";`
- El programa imprime tus valores sin errores.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta de la tarea

Debes estar en la carpeta que contiene `Starter.java`.

Ejemplo:

```bash
cd student-tasks\task-01-hello-variables
```

Comprueba con:

```bash
dir
```

Debes ver `Starter.java`, `TASK.md` y `TASK.es.md`.

### 3) Compila

```bash
javac Starter.java
```

Esto crea `Starter.class`. Si hay errores, corrige el código y vuelve a compilar.

### 4) Ejecuta

```bash
java Starter
```

- Correcto: `java Starter`
- Incorrecto: `java Starter.java`
- Incorrecto: ejecutar desde otra carpeta
