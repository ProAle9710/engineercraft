# Tarea 05 — Cambiar valores y concatenar Strings

Si prefieres inglés, abre `TASK.md`.

## Objetivo
1) Cambiar una variable después de crearla.
2) Unir Strings con `+` para armar un texto más largo.

## Cambiar un valor

```java
int lives = 3;
lives = 2; // ahora es 2
```

## Concatenación (unir texto)

```java
String a = "Welcome";
String b = " player";
String finalText = a + b; // "Welcome player"
```

Puedes unir varias piezas:

```java
String full = "User: " + "Ana" + " | Level: " + 5;
// Resultado: "User: Ana | Level: 5"
```

Los números también se pueden concatenar a un String. Java los convierte en texto.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `message` con Hello (práctica).
2. Crea int `points` en 0 (práctica).
3. Cambia `message` a Welcome (nuevo: reasignar, sin el tipo otra vez).
4. Cambia `points` a 10.
5. Crea String `finalText` concatenando `message` y ` player`.
6. Crea un String más largo `longText` concatenando `finalText`, has, `points` y más texto.
   Idea esperada: Welcome player has 10 points today.

## Listo cuando
`longText` sea un String largo armado con varios `+`.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-05-rename-and-reuse
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
