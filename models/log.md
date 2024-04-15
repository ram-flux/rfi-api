## db

export DATABASE_URL=postgres://my-user:my-secret-pw@127.0.0.1:5432/rfim

sqlx database create --database-url postgres://my-user:my-secret-pw@127.0.0.1:5432/rfim
sqlx database create --database-url postgres://my-user:my-secret-pw@localhost:5432/rfim --verbose


RUST_LOG=debug sqlx database create --database-url postgres://my-user:my-secret-pw@127.0.0.1:5432/rfim
