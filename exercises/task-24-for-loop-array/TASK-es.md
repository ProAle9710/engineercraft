# Tarea 24 — Ciclos: `for` y `while`

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Esta es la lección donde **aprendes ciclos**. Léela despacio.
Vas a usar `for` y `while` para repetir trabajo en vez de copiar las mismas líneas.

## ¿Por qué un ciclo?

Si un arreglo tiene muchos elementos, escribir `names[0]`, `names[1]`, `names[2]`... a mano es lento y fácil de equivocarse.
Un ciclo repite la misma idea para cada índice.

---

## 1) El ciclo `for` (típico cuando sabes cuántas veces)

Usa `for` cuando ya conoces el número de repeticiones: **el length del arreglo**, o "haz esto 5 veces".

### Patrón para memorizar

```java
for (int i = 0; i < names.length; i++) {
    System.out.println(names[i]);
}
```

Los paréntesis tienen **tres partes**, separadas por `;`:

| Parte | Ejemplo | Significado |
|-------|---------|-------------|
| inicio | `int i = 0` | crea el contador; empieza en el primer índice |
| condición | `i < names.length` | sigue **mientras esto sea true** |
| paso | `i++` | después de cada vuelta, pasa al siguiente índice (`i = i + 1`) |

Dentro de `{ }` escribes qué hacer **una vez por índice**.

### Por qué `for` es bueno para arreglos

- Inicio, parada y paso están **en una línea** — cuesta más olvidar `i++`
- Perfecto para recorrer **todos** los elementos de 0 a length-1
- Otros programadores esperan un `for` para este trabajo

---

## 2) El ciclo `while` (típico cuando repites hasta que cambia una condición)

`while` solo pregunta: **¿esto sigue siendo true?**
Tú creas el contador **antes** y lo mueves **dentro** del ciclo.

### Patrón para memorizar

```java
int i = 0;
while (i < names.length) {
    System.out.println(names[i]);
    i++;
}
```

Léelo: *mientras `i` sea un índice válido, imprime, luego súmale 1 a `i`*.

### Importante

Si olvidas `i++` dentro del `while`, la condición nunca se vuelve false. Eso es un **ciclo infinito**. El programa no termina.

`for` pone `i++` en el encabezado, así que ese error es menos común con arreglos.

### Por qué `while` es bueno

- No siempre sabes el conteo de antemano
- Ideas: seguir preguntando hasta que la contraseña sea correcta; seguir sumando mientras el total sea menor que 100
- La condición de parada puede ser cualquier boolean, no solo `i < length`

Ejemplo pequeño (no es un arreglo):

```java
int lives = 3;
while (lives > 0) {
    // jugar
    lives = lives - 1;
}
```

Aquí ciclas **hasta que lives sea 0**. `while` encaja mejor con esa historia.

---

## 3) El mismo trabajo, dos estilos (arreglos)

Estos dos programas hacen **lo mismo**:

```java
// for — elección típica para arreglos
for (int i = 0; i < nums.length; i++) {
    System.out.println(nums[i]);
}

// while — también correcto, pero tú manejas i
int i = 0;
while (i < nums.length) {
    System.out.println(nums[i]);
    i++;
}
```

**Típico:** `for` para todo el arreglo. **Típico:** `while` hasta que una condición cambie.

Eso es un hábito, no una ley.

---

## 4) Cualquier ciclo puede hacer cualquier trabajo

Esto es importante:

- Es **correcto** usar `for` o `while` en **cualquiera** de estas situaciones.
- Con suficiente creatividad, puedes reescribir uno como el otro.
- "Típico" significa *más fácil de leer*, no *la única forma legal*.

### Recorrer un arreglo con `while` (no es lo usual, sigue siendo correcto)

Ya lo viste arriba. Funciona. Solo manejas `i` tú.

### Cuenta regresiva con `for` (no es lo usual, sigue siendo correcto)

`while (lives > 0)` encaja con la historia. Pero este `for` hace lo mismo:

```java
for (int lives = 3; lives > 0; lives = lives - 1) {
    System.out.println(lives);
}
```

Mismo resultado: 3, luego 2, luego 1. No está "mal". Solo es menos natural que `while` para "hasta que se acaben las vidas".

### Buscar con `for` (también correcto)

Puedes recorrer todo el arreglo con `for` aunque ya hayas encontrado el valor.
También puedes usar `while` y parar antes (`while (!found && i < names.length)`).
Las dos son válidas.

**Regla de clase:** elige el ciclo más fácil de leer. Si el profesor pide `for` o `while`, es para practicar, no porque el otro sea ilegal.

---

## 5) Decisión rápida (típico, no obligatorio)

| Situación | Se suele preferir | Por qué |
|-----------|-------------------|---------|
| Visitar cada índice de un arreglo | `for` | inicio, condición e `i++` juntos |
| Sabes el número exacto de repeticiones | `for` | el encabezado muestra el plan |
| Repetir hasta que algo sea false | `while` | la condición es la historia |
| Puedes parar a mitad (búsqueda) | cualquiera; `while` es natural | puedes ciclar mientras `!found` |

Los dos necesitan una condición **boolean**. Los dos pueden ciclar para siempre si la condición nunca se vuelve false.

---

## Dónde trabajar
- Archivo: `Starter.java`

**Abre `Starter.java` primero.** Arriba hay 4 bloques **EXAMPLE** que ya compilan y se ejecutan:
1. `for` sobre un arreglo
2. el **mismo** recorrido con `while`
3. `while` hasta que lives sea 0
4. la **misma** cuenta regresiva con `for` (no típico, igual correcto)

Ejecuta el archivo una vez antes de escribir tus TODO para ver los ejemplos impresos.

## Pasos

1. Crea String[] `cities` con 3 ciudades (práctica).
2. Usa un **`for`** para imprimir cada ciudad.
3. Crea int[] `nums` con 4 números pares.
4. Imprime cada número con un **`while`** (debes escribir `int i`, `while`, print e `i++`).
5. Crea int `lives` en 3.
6. Usa un **`while`**: mientras lives > 0, imprime lives y resta 1.
   Este es el caso típico "hasta que cambia una condición".
7. Haz **la misma cuenta regresiva** con un **`for`** (no típico, pero correcto — sección 4).
   Usa otro nombre de variable, por ejemplo `livesFor`.

## Listo cuando
- Las ciudades se imprimen con `for`
- Los números se imprimen con `while`
- Las vidas imprimen 3, 2, 1 con `while` **y** otra vez con `for`

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-24-for-loop-array
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
