# sqlx-todos

A web-service that manages simplistic todo lists with files.

The goal of this project was to learn how to write a basic http/json web-service with file
upload/download using Rust.

## MinIO

To run a minio server in docker:

```shell
docker run -d -p 9000:9000 -p 9001:9001 --name minio -v .storage/minio:/data minio/minio server /data --console-address ":9001"
```

**References**

- [axum](https://docs.rs/axum/latest/axum/)
- [sqlx](https://docs.rs/sqlx/latest/sqlx/)
- [utoipa](https://docs.rs/utoipa/latest/utoipa/)
