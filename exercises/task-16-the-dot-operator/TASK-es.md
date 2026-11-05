# Tarea 16 — El punto ( . ) en una variable

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Aprender que una variable no es solo una caja con un valor.
En Java, algunos tipos (como `String`) también pueden **hacer acciones**.
Esas acciones se llaman con un **punto** `.`

## Idea nueva: el punto

```java
String name = "Ana";
int size = name.length();
```

Qué pasa:

1. `name` guarda el texto `"Ana"`.
2. El **punto** `.` significa: "usa algo que pertenece a esta variable".
3. `length()` es una **acción** (un **método**) que cuenta los caracteres.
4. Para `"Ana"`, `size` queda en `3`.

Así: `variable.metodo()`

- Izquierda del punto = la variable
- Derecha del punto = la acción

## Importante

- Escribes los paréntesis `()` aunque no haya nada adentro: `length()`
- Esto NO es matemática. El punto no es multiplicación.
- Los `int` no usan `.length()` igual. Esta tarea es de `String`.

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

1. Crea un String `fullName` con tu nombre (ya sabes cómo).
2. Crea int `letterCount` usando `.length()` en `fullName` (nuevo — mira el ejemplo).
3. Crea un String `greeting`.
4. Crea int `greetingSize` usando `.length()` en `greeting`.
5. El print ya está hecho. Revisa que los números tengan sentido.

## Listo cuando
Hayas usado el punto `.` al menos dos veces con `.length()` y el programa corra.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-16-the-dot-operator
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
