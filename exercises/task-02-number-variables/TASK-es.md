# Tarea 02 — Variables numéricas

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Crear variables **int** (números enteros) y usarlas en un cálculo pequeño.

## Sintaxis correcta
Los números **no** llevan comillas. Escribe el tipo antes del nombre:

```java
int age = 18;
int heightCm = 170;
```

## Dónde trabajar
- Archivo: `Starter.java`
- Solo en las secciones `TODO` dentro de `main`.

## Pasos

1. Crea `int age = ...;`
2. Crea `int heightCm = ...;`
3. Crea `int shoesOwned = ...;`
4. Crea `int totalItems = shoesOwned + 2;` (compraste 2 zapatos más).

## Consejo
Puedes crear un valor nuevo a partir de otras variables:

```java
int total = a + b;
```

## Listo cuando
- Todas las variables usen `int name = value;`
- `totalItems` sea `shoesOwned + 2`.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-02-number-variables
```

Comprueba con `dir`.

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
