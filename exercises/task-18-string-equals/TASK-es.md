# Tarea 18 — Comparar Strings con .equals

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Aprender la forma correcta de comparar dos `String` en Java.

## ¿Por qué no usar == con texto?

Para principiantes, esta regla:

- Usa `==` para números (`int`) y para `boolean`
- Usa `.equals(...)` para texto `String`

```java
String a = "yes";
String b = "yes";
boolean same = a.equals(b); // true
```

Léelo: "pregúntale a la variable `a` si es igual al valor de `b`".

## Texto vacío

```java
boolean isEmpty = note.equals("");
```

`""` es un String con cero caracteres.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea String `password`.
2. Crea String `typed` (luego cámbialo para probar).
3. Crea boolean `isCorrect` usando `.equals` (nuevo — mira el ejemplo).
4. Crea String `note` (prueba vacío y luego con texto).
5. Crea boolean `noteIsEmpty` usando `.equals` con texto vacío.
6. Asigna String `status` con if/else:
   - Correcto → Access granted
   - Si no → Access denied

## Listo cuando
Hayas usado `.equals` (no `==`) para comparar Strings, y el if/else funcione.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-18-string-equals
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
