# add deps

```bash
cargo cargo add argon2 async-trait dotenv jsonwebtoken time lettre tracing-subscriber

cargo add serde --features derive
cargo add serde_json
cargo add chrono --features serde
cargo add sqlx --features runtime-async-std-native-tls,postgres,chrono,uuid
cargo add axum
cargo add axum-extra --features cookie
cargo add tokio --features full
cargo add tower-http --features cors,trace
cargo add tracing-subscriber
cargo add uuid --features=serde,v4
cargo add validator --features=derive
```
