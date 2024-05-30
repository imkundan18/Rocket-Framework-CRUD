# Rocket-Framework-CURD
# CURD opration in Rust Using Rocket framework
## Using Rocket framework with mongoDB

Using mongoDB for Data Handle

- Post - Insert Data
- Get - Get Data
- Put - Update Data
- Delete - Delete Data

Using Cargo for Package Manager in Rust

#### [dependencies]
rocket = { version = "0.5.0-rc.2", features = ["json"] }
mongodb = "2.1"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
futures = "0.3"
bson = "2.0"
