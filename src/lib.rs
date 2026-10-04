use std::sync::Arc;

use serde_json::{Value, from_str, to_string};
use surrealdb::{Surreal, engine::local::{Db, SurrealKv}};
use tokio::runtime::{Builder, Runtime};

#[derive(Debug,thiserror::Error)]
enum BridgeError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Query error: {0}")]
    Query(String),
    #[error("Serialization error:{0}")]
    Serialization(String),
    #[error("Not Connected: {0}")]
    NotConnected(String)
}

struct SurrealdbBridge {
    db: Surreal<Db>,
    _runtime: Arc<Runtime>,
}

impl SurrealdbBridge {
    pub fn new(
        path:String,
        namespace:String,
        database:String
    ) -> Result<Self,BridgeError>{
        // 1. crear runtime de tokio
        let runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|e| BridgeError::Config(format!("runtime: {}",e)))?;
        
        // 2. crear la conexion db dentro del runtime 
        let db = runtime.block_on(async {
            let db = Surreal::new::<SurrealKv>(&path)
                .await
                .map_err(|e| BridgeError::Connection(e.to_string()))?;
            
            db.use_ns(namespace)
                .use_db(database)
                .await
                .map_err(|e| BridgeError::Config(e.to_string()))?;
            
            Ok::<_,BridgeError>(db)
        })?;
        
        Ok(
            Self {
                db,
                _runtime:Arc::new(runtime)
            }
        )
    }
    
    pub async fn insertar(
        &self,
        table:String,
        json_data:String
    ) -> Result<String,BridgeError> {
        let data:Value = from_str(&json_data).map_err(|e| BridgeError::Serialization(e.to_string()))?;
        let query = format!("CREATE {} CONTENT $data",table);
        let mut response = self.db
            .query(query)
            .bind(("data",data))
            .await.map_err(|e| BridgeError::Query(e.to_string()))?;
        
        
        let result:Option<Value> = response.take(0).map_err(|e| BridgeError::Query(e.to_string()))?;
        
        to_string(&result).map_err(|e| BridgeError::Serialization(e.to_string()))
    }
    
    pub async fn query_dynamic(
        &self,
        surql:String
    ) -> Result<String,BridgeError> {
        let mut response = self.db
            .query(surql)
            .await
            .map_err(|e| BridgeError::Query(e.to_string()))?;
            
        
        let result: Vec<Value> = response.take(0).map_err(|e| BridgeError::Query(e.to_string()))?;
        
        to_string(&result)
            .map_err(|e| BridgeError::Serialization(e.to_string()))
    }
}

///TEST

#[cfg(test)]
mod tests {
    use serde_json::{Value, from_str};
use tempfile::tempdir;
use tokio::runtime::Runtime;

use crate::SurrealdbBridge;

    
    #[test]
    fn test_new_creates_database(){
        let dir = tempdir().unwrap();
        let path = dir.path().to_string_lossy().to_string();
        
        let result = SurrealdbBride::new(
            path,
            "test_ns".to_string(),
            "test_db".to_string(),
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_insertart(){
        let dir = tempdir().unwrap();
        let path = dir.path().to_string_lossy().to_string();
        
        let bridge = SurrealdbBride::new(
            path,
            "test_ns".to_string(),
            "test_db".to_string()
        )
        .unwrap();
        
        let Runtime = Runtime::new().unwrap();
        
        let result = Runtime.block_on(async {
            bridge.insertar("users".to_string(), r#"{
                "name":"wuilmer",
                "age":37
            }"#.to_string()).await
        });
        
        assert!(result.is_ok());
        
        let data = result.unwrap();
        
        let value:Value = from_str(&data).unwrap();
        
        assert!(value.is_object());
    }
    
    #[test]
    fn test_query_dinamyc(){
        let dir = tempdir().unwrap();
        let path = dir.path().to_string_lossy().to_string();
        
        let bridge = SurrealdbBride::new(
            path,
            "test_ns".to_string(),
            "test_db".to_string()
        )
        .unwrap();
        
        let Runtime = Runtime::new().unwrap();
        
        let result = Runtime.block_on(async {
            bridge.query_dynamic(
                r#"
                    CREATE users CONTENT {
                        name:"wuilmerj24",
                        age:37
                    }
                "#.to_string()
            )
            .await
        });
        
        assert!(result.is_ok());
        
        let data = result.unwrap();
        let Value:Value = from_str(&data).unwrap();
        
        assert!(Value.is_array());
    }
}