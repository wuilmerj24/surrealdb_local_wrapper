# SurrealDB Local Wrapper

Librería escrita en **Rust** que permite trabajar con una base de datos local utilizando **SurrealDB + SurrealKV**.

Este proyecto forma parte de un proyecto práctico donde construiremos progresivamente una librería reutilizable para trabajar con una base de datos local desde diferentes plataformas.

## Tecnologías

* **Rust**
* **SurrealDB**
* **SurrealKV**
* **Tokio**
* **Serde / Serde JSON**
* **thiserror**

## Características actuales

En esta primera etapa la librería permite:

* Crear y abrir una base de datos local.
* Configurar namespace y database.
* Ejecutar operaciones de escritura.
* Insertar datos utilizando JSON.
* Ejecutar consultas dinámicas utilizando SurrealQL.
* Trabajar de forma asíncrona con Tokio.
* Utilizar un runtime Tokio multi-thread.
* Convertir los resultados de SurrealDB a JSON.
* Manejar errores mediante un tipo de error propio.

## Arquitectura

La librería utiliza un runtime de Tokio para ejecutar las operaciones asíncronas de SurrealDB.

```text
                 SurrealdbBridge
                       │
              ┌────────┴────────┐
              │                 │
           Runtime            Database
            Tokio          Surreal<Db>
              │                 │
              │            SurrealKV
              │                 │
              └────────┬────────┘
                       │
                  Base de datos
                     local
```

El runtime se crea utilizando Tokio en modo multi-thread:

```rust
Builder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    .build()
```

Esto permite que las operaciones asíncronas de la base de datos puedan ejecutarse dentro de un runtime con múltiples workers.

## Estructura principal

La librería actualmente está contenida en `lib.rs`.

La estructura principal es:

```rust
pub struct SurrealdbBridge {
    db: Surreal<Db>,
    _runtime: Arc<Runtime>,
}
```

`SurrealdbBridge` mantiene:

* Una instancia de la base de datos SurrealDB.
* Una referencia compartida al runtime de Tokio.

## Crear una conexión

La instancia se crea utilizando:

```rust
SurrealdbBridge::new(
    path,
    namespace,
    database
)
```

Por ejemplo:

```rust
let db = SurrealdbBridge::new(
    "./data/database.db".to_string(),
    "local".to_string(),
    "main".to_string(),
)?;
```

Internamente se crea una instancia de `SurrealKv`:

```rust
let db = Surreal::new::<SurrealKv>(&path).await?;
```

Después se configura el namespace y la database:

```rust
db.use_ns(namespace)
    .use_db(database)
    .await?;
```

## Insertar datos

La librería proporciona el método:

```rust
insertar(
    table: String,
    json_data: String
)
```

El método recibe los datos como JSON.

Por ejemplo:

```json
{
    "nombre": "Juan",
    "edad": 30
}
```

Internamente el JSON se convierte a `serde_json::Value`:

```rust
let data: Value = from_str(&json_data)?;
```

Después se genera una consulta SurrealQL:

```sql
CREATE usuario CONTENT $data
```

Y los datos se envían mediante `bind`:

```rust
self.db
    .query(query)
    .bind(("data", data))
    .await?;
```

Utilizar `bind` permite separar la consulta de los datos que se envían a SurrealDB.

## Consultas dinámicas

También existe el método:

```rust
query_dynamic(
    surql: String
)
```

Este método permite ejecutar directamente una consulta SurrealQL.

Por ejemplo:

```sql
SELECT * FROM usuario;
```

La consulta se ejecuta mediante:

```rust
self.db.query(surql).await?;
```

El resultado se convierte posteriormente a JSON para poder devolverlo como `String`.

## Manejo de errores

El proyecto utiliza [`thiserror`](https://crates.io/crates/thiserror) para definir un tipo de error propio:

```rust
enum BridgeError {
    Connection(String),
    Config(String),
    Query(String),
    Serialization(String),
    NotConnected(String),
}
```

Esto permite diferenciar diferentes tipos de errores relacionados con:

* Conexión.
* Configuración.
* Consultas.
* Serialización.
* Estado de conexión.

Los errores generados por SurrealDB se convierten al tipo `BridgeError`.

Por ejemplo:

```rust
.map_err(|e| BridgeError::Query(e.to_string()))
```

## Dependencias

Las principales dependencias utilizadas son:

```toml
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
surrealdb = { version = "3.2.4", features = ["kv-mem", "kv-surrealkv"] }
thiserror = "2.0.20"
tokio = { version = "1.53.1", features = ["rt-multi-thread", "macros", "tracing"] }
```

### SurrealDB

Se utilizan los motores locales:

* `kv-surrealkv` para almacenamiento persistente.
* `kv-mem` para almacenamiento en memoria cuando sea necesario.

## Requisitos

Para trabajar con el proyecto necesitas tener instalado:

* Rust
* Cargo

Puedes comprobar la instalación con:

```bash
rustc --version
cargo --version
```

## Compilar el proyecto

Desde la raíz del proyecto:

```bash
cargo build
```

Para compilar en modo release:

```bash
cargo build --release
```

## Verificar el proyecto

Puedes comprobar que el proyecto compila correctamente utilizando:

```bash
cargo check
```

## Estado del proyecto

Esta rama corresponde a la **primera etapa del proyecto**.

Actualmente contiene la implementación base de la librería:

```text
Rust
  │
  ├── Tokio
  │
  └── SurrealDB
        │
        └── SurrealKV
              │
              └── Base de datos local
```

En las siguientes etapas se incorporarán nuevas funcionalidades y mejoras sobre esta implementación.

## Rama

Esta versión corresponde a:

```text
main
```

La rama `main` representa la implementación base de la librería antes de incorporar las siguientes etapas del proyecto.
