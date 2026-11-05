# Tarea 17 — Los String están hechos de caracteres (char[])

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Entender que un `String` es como un **arreglo de caracteres**.
Vas a usar `.charAt(index)` y también `.toCharArray()`.

## Idea

```text
String word = "Java";
índice:        0 1 2 3
chars:         J a v a
```

## Dos formas de leer caracteres

### 1) Un carácter con charAt

```java
char first = word.charAt(0); // 'J'
```

### 2) Convertir todo el String en un arreglo real de char

```java
char[] letters = word.toCharArray();
// letters[0] es 'J'
// letters[1] es 'a'
// letters.length es 4
```

Ahora puedes tratar el texto casi como un arreglo normal.

## char vs String

```java
char letter = 'J';      // UN carácter, comillas simples '
String text = "J";      // texto, comillas dobles "
```

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `word` con el texto Java.
2. Crea int `size` usando `.length()` (práctica de la Tarea 16).
3. Crea char `first` usando `.charAt(0)` (nuevo — mira el ejemplo).
4. Crea char `last` usando `.charAt` en el último índice (length - 1).
5. Crea `char[] letters` usando `.toCharArray()` (nuevo — mira el ejemplo).
6. Crea char `secondFromArray` del índice 1 de `letters`.
7. Crea int `arraySize` desde `letters.length` (sin paréntesis).

## Listo cuando
Hayas usado `charAt` y `toCharArray`, y puedas decir:
"un String puede convertirse en un `char[]`."

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-17-string-methods
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
