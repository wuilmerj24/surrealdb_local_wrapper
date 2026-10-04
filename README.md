# SurrealDB Local Wrapper — Tests

Esta rama agrega **pruebas automatizadas** a la librería `surrealdb_local_wrapper`.

Partimos de la implementación disponible en `main` y agregamos tests para comprobar que las principales operaciones de la librería funcionan correctamente.

## Objetivo

Los tests permiten verificar automáticamente que:

* La base de datos puede inicializarse correctamente.
* Los datos pueden insertarse utilizando `insertar()`.
* Las consultas dinámicas pueden ejecutarse utilizando `query_dynamic()`.
* Los resultados devueltos pueden convertirse correctamente a JSON.
* Cada prueba puede trabajar con una base de datos temporal independiente.

## Tecnologías utilizadas

Además de las dependencias utilizadas en `main`, esta etapa utiliza:

* **Rust testing framework**
* **Tokio**
* **tempfile**
* **serde_json**

## Estructura de los tests

Los tests se encuentran dentro de:

```text
src/
└── lib.rs
    └── #[cfg(test)]
        └── mod tests
```

La estructura básica es:

```rust
#[cfg(test)]
mod tests {
    // tests...
}
```

El atributo:

```rust
#[cfg(test)]
```

hace que este módulo solamente sea compilado cuando se ejecutan los tests.

---

# Test de creación de la base de datos

El primer test comprueba que `SurrealdbBridge::new()` pueda crear correctamente una base de datos.

```rust
#[test]
fn test_new_creates_database() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_string_lossy().to_string();

    let result = SurrealdbBridge::new(
        path,
        "test_ns".to_string(),
        "test_db".to_string(),
    );

    assert!(result.is_ok());
}
```

## Base de datos temporal

Para evitar crear archivos permanentes durante las pruebas utilizamos `tempfile`:

```rust
let dir = tempdir().unwrap();
```

Esto crea un directorio temporal que puede utilizarse durante la prueba.

Después obtenemos su ruta:

```rust
let path = dir.path().to_string_lossy().to_string();
```

De esta manera cada ejecución del test utiliza su propio directorio temporal.

Esto es importante porque los tests no deberían depender de archivos o datos creados por ejecuciones anteriores.

## `assert!`

Finalmente comprobamos que la operación haya sido exitosa:

```rust
assert!(result.is_ok());
```

Si `SurrealdbBridge::new()` devuelve un `Ok`, el test pasa.

Si devuelve un `Err`, el test falla.

---

# Test de inserción

El segundo test comprueba que podemos insertar información utilizando el método:

```rust
insertar()
```

Ejemplo:

```rust
#[test]
fn test_insertar() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_string_lossy().to_string();

    let bridge = SurrealdbBridge::new(
        path,
        "test_ns".to_string(),
        "test_db".to_string(),
    )
    .unwrap();

    let runtime = Runtime::new().unwrap();

    let result = runtime.block_on(async {
        bridge
            .insertar(
                "users".to_string(),
                r#"{
                    "name": "wuilmer",
                    "age": 37
                }"#
                .to_string(),
            )
            .await
    });

    assert!(result.is_ok());

    let data = result.unwrap();

    let value: Value = from_str(&data).unwrap();

    assert!(value.is_object());
}
```

## ¿Por qué necesitamos Tokio?

El método `insertar()` es asíncrono:

```rust
pub async fn insertar(...)
```

Por lo tanto, necesitamos ejecutar ese `Future` dentro de un runtime de Tokio.

Creamos un runtime:

```rust
let runtime = Runtime::new().unwrap();
```

Y ejecutamos la operación utilizando:

```rust
runtime.block_on(async {
    // operación async
});
```

Esto permite ejecutar código asíncrono desde nuestro test, que utiliza la función convencional:

```rust
#[test]
```

## Comprobación del resultado

Primero comprobamos que la operación haya sido exitosa:

```rust
assert!(result.is_ok());
```

Después obtenemos el resultado:

```rust
let data = result.unwrap();
```

Como la librería devuelve un `String` que contiene JSON, lo convertimos nuevamente a `serde_json::Value`:

```rust
let value: Value = from_str(&data).unwrap();
```

Finalmente comprobamos que el resultado sea un objeto JSON:

```rust
assert!(value.is_object());
```

---

# Test de consultas dinámicas

El tercer test comprueba el método:

```rust
query_dynamic()
```

Este método permite ejecutar una consulta SurrealQL directamente.

```rust
#[test]
fn test_query_dynamic() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_string_lossy().to_string();

    let bridge = SurrealdbBridge::new(
        path,
        "test_ns".to_string(),
        "test_db".to_string(),
    )
    .unwrap();

    let runtime = Runtime::new().unwrap();

    let result = runtime.block_on(async {
        bridge
            .query_dynamic(
                r#"
                    CREATE users CONTENT {
                        name: "wuilmerj24",
                        age: 37
                    }
                "#
                .to_string(),
            )
            .await
    });

    assert!(result.is_ok());

    let data = result.unwrap();

    let value: Value = from_str(&data).unwrap();

    assert!(value.is_array());
}
```

En este caso ejecutamos:

```sql
CREATE users CONTENT {
    name: "wuilmerj24",
    age: 37
}
```

El resultado de `query_dynamic()` se devuelve como JSON.

Por eso primero verificamos:

```rust
assert!(result.is_ok());
```

Después convertimos el resultado:

```rust
let value: Value = from_str(&data).unwrap();
```

Y comprobamos que sea un array:

```rust
assert!(value.is_array());
```

Esto corresponde con la implementación actual de `query_dynamic()`, donde el resultado se obtiene como:

```rust
let result: Vec<Value> = response
    .take(0)
    .map_err(|e| BridgeError::Query(e.to_string()))?;
```

---

# Ejecutar los tests

Para ejecutar todos los tests del proyecto:

```bash
cargo test
```

Cargo compilará el proyecto y ejecutará automáticamente las funciones marcadas con:

```rust
#[test]
```

También puedes utilizar:

```bash
cargo test -- --nocapture
```

si quieres mostrar en la terminal los mensajes enviados mediante `println!()` durante las pruebas.

## Ejecutar un test específico

Puedes ejecutar solamente un test utilizando su nombre:

```bash
cargo test test_new_creates_database
```

Por ejemplo:

```bash
cargo test test_insertar
```

o:

```bash
cargo test test_query_dynamic
```

---

# ¿Por qué utilizar una base de datos temporal?

Los tests deben ser lo más independientes posible.

En lugar de utilizar una base de datos fija como:

```text
./database.db
```

utilizamos:

```rust
tempdir()
```

Esto proporciona un directorio temporal para cada prueba.

La idea es:

```text
Test 1
  │
  └── Base de datos temporal A

Test 2
  │
  └── Base de datos temporal B

Test 3
  │
  └── Base de datos temporal C
```

De esta forma una prueba no debería modificar los datos utilizados por otra.

---

# Flujo de las pruebas

La estrategia utilizada en esta etapa es:

```text
         cargo test
              │
              ▼
      ┌───────────────┐
      │  Crear tempdir │
      └───────┬───────┘
              │
              ▼
     SurrealdbBridge::new()
              │
              ▼
       Base de datos local
              │
       ┌──────┴──────┐
       ▼             ▼
   insertar()   query_dynamic()
       │             │
       ▼             ▼
     JSON          JSON
       │             │
       └──────┬──────┘
              ▼
           assert!
              │
              ▼
          Test pasa
```

# Estado del proyecto

Esta rama parte de:

```text
main
```

y agrega:

```text
main
  │
  └── tests
       │
       ├── Test de creación
       ├── Test de inserción
       └── Test de consultas dinámicas
```

La rama `tests` representa la versión del proyecto con pruebas automatizadas antes de incorporar la siguiente etapa de integración con **UniFFI**.

## Rama

```text
tests
```

Para volver a la implementación base:

```bash
git checkout main
```

Para utilizar la versión con tests:

```bash
git checkout tests
```
