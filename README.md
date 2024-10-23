## Runbook

NOTE: This runbook assumes you already have the project git-cloned locally.

1) Add required Shuttle dependencies to your project:
```bash
cargo add shuttle-runtime shuttle-axum shuttle-shared-db \
-F shuttle-shared-db/postgres
```

2) The `#[tokio::main]` macro needs to be replaced with `#[shuttle_runtime::main]`:

```rust
#[shuttle_runtime::main]
async fn main() // ... etc
```

3) The `async fn main()` function (in `src/main.rs`) needs to return `shuttle_axum::ShuttleAxum`. It should look like this:
```rust
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // .. function code here
}
```

4) Remove the TcpListener/serve code and return the router:

```rust
// delete this..

let tcp = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("to create a TcpListener");

    axum::serve(tcp, router)
        .await
        .expect("to serve an Axum router");

// .. and replace it with this

Ok(router.into())
```

5) Add the database annotation and replace the environment variable with it:

```rust
// .. original code
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let conn_string = std::env::var("DATABASE_URL").expect("DATABASE_URL env var to exist");
    let state = AppState::new(conn_string).await;
    // .. rest of code
}

// .. new code

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] conn_string: String,
) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new(conn_string).await;
    // .. rest of code
}
```

6) Use `shuttle run` to ensure it runs locally. Make sure Docker is installed!

7) Use `shuttle deploy --ad` to deploy your project.
