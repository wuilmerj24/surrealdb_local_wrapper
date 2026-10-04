# SurrealDB Local Wrapper — UniFFI

Esta rama agrega **UniFFI** al proyecto `surrealdb_local_wrapper`.

Partimos de la implementación desarrollada en las ramas anteriores y agregamos una interfaz utilizando **WebIDL (UDL)** para generar bindings que permitan utilizar la librería Rust desde otros lenguajes.

## Objetivo

El objetivo de esta etapa es exponer la funcionalidad principal de la librería Rust mediante **UniFFI**.

La implementación sigue manteniéndose en Rust:

```text
Rust
 │
 ├── SurrealDB
 ├── Tokio
 └── SurrealdbBridge
          │
          ▼
        UniFFI
          │
     ┌────┴────┐
     ▼         ▼
  Kotlin     Swift
```

UniFFI se encarga de generar el código necesario para que la API pública de Rust pueda ser utilizada desde otros lenguajes.

## Tecnologías

* **Rust**
* **SurrealDB**
* **SurrealKV**
* **Tokio**
* **UniFFI**
* **UDL / WebIDL**
* **Kotlin**
* **Swift**

---

# Estructura del proyecto

En esta etapa se agregan varios archivos relacionados con UniFFI:

```text
surrealdb_local_wrapper/
│
├── src/
│   ├── lib.rs
│   └── lib.udl
│
├── build.rs
├── uniffi-bindgen.rs
├── uniffi.toml
├── Cargo.toml
└── README.md
```

Cada archivo tiene una responsabilidad específica.

---

# UDL — `lib.udl`

El archivo:

```text
src/lib.udl
```

define la interfaz que UniFFI debe exponer.

Actualmente contiene:

```udl
namespace surrealdb_local_wrapper {};

[Error]
enum BridgeError {
    "Connection",
    "Config",
    "Query",
    "Serialization",
    "NotConnected"
};

interface SurrealdbBridge {
    [Throws=BridgeError]
    constructor(string path, string namespace, string database);

    [Async, Throws=BridgeError]
    string insertar(string table, string json_data);

    [Async, Throws=BridgeError]
    string query_dynamic(string surql);
};
```

## Namespace

El namespace:

```udl
namespace surrealdb_local_wrapper {};
```

define el espacio de nombres utilizado por la interfaz UDL.

---

# Exposición de errores

El `enum`:

```udl
[Error]
enum BridgeError {
    "Connection",
    "Config",
    "Query",
    "Serialization",
    "NotConnected"
};
```

define los errores que pueden ser propagados a través de la interfaz UniFFI.

Esto permite que los errores definidos en Rust tengan una representación que pueda ser utilizada desde los lenguajes soportados por UniFFI.

La definición UDL corresponde al tipo de error utilizado en Rust:

```rust
enum BridgeError {
    Connection(String),
    Config(String),
    Query(String),
    Serialization(String),
    NotConnected(String),
}
```

La capa UDL define los valores que estarán disponibles a través de la API generada.

---

# Interfaz `SurrealdbBridge`

La interfaz:

```udl
interface SurrealdbBridge {
    ...
};
```

define qué funcionalidades de `SurrealdbBridge` serán visibles desde los bindings generados.

No es necesario exponer toda la implementación interna de Rust.

En esta etapa se exponen:

* Constructor.
* `insertar()`.
* `query_dynamic()`.

---

# Constructor

El constructor se define mediante:

```udl
[Throws=BridgeError]
constructor(string path, string namespace, string database);
```

Esto indica que el constructor puede producir un `BridgeError`.

Los parámetros son:

```text
path
namespace
database
```

y corresponden a la configuración necesaria para abrir la base de datos local.

---

# Métodos asíncronos

Los métodos:

```udl
[Async, Throws=BridgeError]
string insertar(string table, string json_data);

[Async, Throws=BridgeError]
string query_dynamic(string surql);
```

están marcados con:

```udl
[Async]
```

porque en Rust corresponden a métodos asíncronos:

```rust
pub async fn insertar(...)
```

y:

```rust
pub async fn query_dynamic(...)
```

UniFFI utiliza esta información para generar una interfaz adecuada para operaciones asíncronas en los bindings.

Además:

```udl
[Throws=BridgeError]
```

indica que estas operaciones pueden producir un error.

---

# `build.rs`

El proyecto utiliza un `build.rs` para generar el scaffolding de UniFFI:

```rust
use uniffi::generate_scaffolding;

fn main() {
    generate_scaffolding("./src/lib.udl").unwrap();
}
```

Cargo ejecuta `build.rs` durante el proceso de compilación.

La función:

```rust
generate_scaffolding()
```

utiliza el archivo UDL para generar el código necesario para conectar la interfaz definida en `lib.udl` con la implementación Rust.

El flujo es:

```text
lib.udl
   │
   ▼
build.rs
   │
   ▼
generate_scaffolding()
   │
   ▼
Scaffolding UniFFI
   │
   ▼
Implementación Rust
```

---

# `uniffi-bindgen.rs`

El proyecto también contiene:

```text
uniffi-bindgen.rs
```

con:

```rust
use uniffi::uniffi_bindgen_main;

fn main() {
    uniffi_bindgen_main();
}
```

Este binario permite utilizar el comando de generación de bindings proporcionado por UniFFI.

En `Cargo.toml` se registra como:

```toml
[[bin]]
name = "uniffi-bindgen"
path = "uniffi-bindgen.rs"
```

Por lo tanto, Cargo conoce este archivo como un binario adicional del proyecto.

---

# `uniffi.toml`

El archivo:

```text
uniffi.toml
```

contiene la configuración para los bindings.

## Kotlin

```toml
[bindings.kotlin]
package_name = "wuilmerj24.dev.surrealdb_local_wrapper"
cdylib_name = "surrealdb_local_wrapper"
android = true
```

Esta configuración indica:

* El package que utilizarán los bindings Kotlin.
* El nombre de la librería nativa.
* Que la configuración está preparada para Android.

El resultado será una API Kotlin generada a partir de la interfaz definida en `lib.udl`.

## Swift

```toml
[bindings.swift]
cdylib_name = "surrealdb_local_wrapper"
module_name = "surrealdb_local_wrapper"
```

Esta sección configura la generación de bindings para Swift.

El módulo generado utilizará:

```text
surrealdb_local_wrapper
```

como nombre del módulo.

---

# Configuración de Cargo

UniFFI se agrega como dependencia:

```toml
uniffi = {
    version = "0.32.1",
    features = [
        "bindgen",
        "build",
        "cli",
        "ffi-trace",
        "tokio"
    ]
}
```

También se utiliza como dependencia de compilación:

```toml
[build-dependencies]
uniffi = {
    version = "0.32.1",
    features = ["build"]
}
```

Y para los tests relacionados con UniFFI:

```toml
[dev-dependencies]
uniffi = {
    version = "0.32.1",
    features = ["bindgen-tests"]
}
```

---

# Tipos de librería

El proyecto configura tres tipos de librería:

```toml
[lib]
crate-type = ["lib", "cdylib", "staticlib"]
```

Esto permite generar diferentes formatos de la librería Rust.

### `lib`

La librería Rust convencional.

### `cdylib`

Una librería dinámica preparada para ser utilizada mediante FFI.

### `staticlib`

Una librería estática que puede ser enlazada desde otros lenguajes o proyectos.

La generación de bindings utiliza estos artefactos junto con el código generado por UniFFI.

---

# Flujo de generación

El flujo general de esta etapa es:

```text
                 lib.udl
                    │
                    ▼
             Definición de API
                    │
                    ▼
              UniFFI scaffolding
                    │
                    ▼
                  Rust
                    │
              ┌─────┴─────┐
              ▼           ▼
           Kotlin       Swift
              │           │
              ▼           ▼
           Android        iOS
```

La idea principal es que la API se define de forma explícita en `lib.udl` y UniFFI utiliza esa definición para generar los bindings correspondientes.

---

# API expuesta

La API disponible mediante UniFFI en esta etapa es:

```text
SurrealdbBridge
│
├── constructor(path, namespace, database)
│
├── insertar(table, json_data)
│
└── query_dynamic(surql)
```

## Constructor

```text
SurrealdbBridge(path, namespace, database)
```

Crea la instancia de la base de datos.

## Insertar

```text
insertar(table, json_data)
```

Permite insertar información utilizando JSON.

## Consulta dinámica

```text
query_dynamic(surql)
```

Permite ejecutar consultas SurrealQL directamente.

---

# Separación entre Rust y UniFFI

Una de las ideas importantes de esta arquitectura es separar:

```text
Implementación
     │
     ▼
   Rust
     │
     │
     ▼
Interfaz pública
     │
     ▼
   UDL
     │
     ▼
  UniFFI
     │
 ┌───┴───┐
 ▼       ▼
Kotlin  Swift
```

Rust contiene la lógica de la aplicación y el acceso a SurrealDB.

UDL define qué parte de esa lógica será expuesta.

UniFFI genera los bindings necesarios para consumir esa API desde otros lenguajes.

---

# Compilar el proyecto

Para comprobar que el proyecto compila correctamente:

```bash
cargo check
```

Para compilar:

```bash
cargo build
```

Para una compilación de producción:

```bash
cargo build --release
```

---

# Estado del proyecto

Esta rama representa la etapa en la que la librería Rust se prepara para ser utilizada desde otros lenguajes mediante UniFFI.

La evolución del proyecto queda:

```text
main
 │
 │ Rust + SurrealDB
 ▼
tests
 │
 │ + Tests
 ▼
uniffi
 │
 │ + UniFFI
 ▼
Bindings
 │
 ├── Kotlin
 └── Swift
```

## Rama

```text
uniffi
```

Esta rama contiene la integración de UniFFI sobre la implementación desarrollada en las ramas anteriores.
