# Tarea 08 — Config de Firebase como variables

Si prefieres inglés, abre `TASK.md`.

## Objetivo
Guardar valores falsos de configuración de Firebase en variables **String** y **boolean**.
No necesitas una cuenta real de Firebase.

## Ejemplos de sintaxis

```java
String apiKey = "DEMO_API_KEY_123";
boolean isDemoMode = true;
```

## Dónde trabajar
- Archivo: `Starter.java`

## Pasos

Escribe cada variable tú (práctica de String y boolean).

1. Crea String `apiKey` con el texto DEMO_API_KEY_123.
2. Crea String `authDomain` con demo-project.firebaseapp.com.
3. Crea String `projectId` con demo-project.
4. Crea String `storageBucket` con demo-project.appspot.com.
5. Crea String `appId` con 1:000000:web:abcdef.
6. Crea boolean `isDemoMode` en true.

## Nota de seguridad
En clase solo usamos valores **demo / falsos**.

## Listo cuando
Todas las variables de config existan con tipos Java correctos.

## Cómo ejecutar

Sigue estos pasos. No ejecutes los comandos desde una carpeta cualquiera.

### 1) Abre una consola (terminal)

- **VS Code**: menú **Terminal → New Terminal**
- **IntelliJ IDEA**: menú **View → Tool Windows → Terminal**
- O abre **PowerShell** / **Símbolo del sistema** en Windows

### 2) Entra a ESTA carpeta

```bash
cd student-tasks\task-08-firebase-config-vars
```

### 3) Compila: `javac Starter.java`
### 4) Ejecuta: `java Starter` (no `java Starter.java`)
