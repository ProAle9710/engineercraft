# Tarea 31 — Ternario vs if/else (diferencias y límites)

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Entender **cuándo usar** un ternario (`? :`) y **cuándo usar** `if / else`.
Están relacionados, pero no son la misma herramienta.

## 1) Misma idea, forma distinta

### if / else (sentencias — hacen acciones)

```java
String status;
if (didPass) {
    status = "Pass";
} else {
    status = "Fail";
}
```

### Ternario (expresión — da un valor)

```java
String status = didPass ? "Pass" : "Fail";
```

Los dos necesitan una condición **boolean**.

## 2) Para qué sirve bien el ternario

Úsalo cuando:
- Eliges **un valor** entre dos opciones
- El código queda **corto y claro**
- Asignas ese valor a una variable

```java
int fee = isMember ? 0 : 10;
String label = isOnline ? "Online" : "Offline";
```

## 3) Límites del ternario

El ternario **NO** es bueno cuando:

1. Necesitas **muchas líneas** en cada rama
2. Hay **más de dos** caminos claros (usa `if / else if / else`)
3. La línea se vuelve difícil de leer (ternarios anidados)

Malo para principiantes:

```java
String x = a ? (b ? "1" : "2") : "3";
```

Usa `if / else` normal.

## 4) Límites del if / else

Es más flexible, pero:
- Es más largo para una elección simple de un valor
- Es una **sentencia**: no "se convierte" solo en un valor
- Para `"Pass"` / `"Fail"`, el ternario suele ser más claro

## 5) Tabla rápida

| Situación | Prefiere |
|-----------|----------|
| Elegir un valor simple | Ternario `? :` |
| Varias acciones en una rama | `if / else` |
| 3 o más caminos | `if / else if / else` |
| El código se está poniendo confuso | Prefiere `if / else` |

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

Los ejemplos de arriba enseñan la idea. En `Starter.java` escribe el código tú.

1. Crea boolean `isLoggedIn` (luego prueba false).
2. Crea boolean `hasProfile`.
3. Crea String `gate` con un ternario (Enter vs Login required).
4. Crea int `bonus` con un ternario (5 vs 0).
5. Usa **if / else** (no ternario) porque cambias DOS variables:
   - Si hasProfile: Profile OK y ready true
   - Si no: Create profile y ready false
6. Crea un String largo `summary` concatenando.
7. Crea String `finalLine` con un ternario (GO vs WAIT).

## Listo cuando
Puedas explicar: ternario = elegir un **valor**; if/else = elegir **acciones**. No anides ternarios todavía.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-31-ternary-vs-if
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
