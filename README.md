# bazel_rust

System: 
* BAZEL_VERSION=7.1.1
* RUST_VERSION = "1.76.0"
* OS: MacOS Sonoma 14.4
* Apple M3 Max (ARM64)

Reproduced an error in a Bazel configuration for Rust monorepo

1) Sync dependencies from Cargo to Bazel

>  CARGO_BAZEL_ISOLATED=false CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index

2) Try Bazel build

> bazel build //:build

Error:
```Bash
no such target '@@crate_index__async-executor-1.8.0//:executor': target 'executor' not declared in package '' 
defined by /private/var/tmp/_bazel_marvin/9cf0c27c2f651654c121c526ef57328f/external/crate_index__async-executor-1.8.0/BUILD.bazel

ERROR: /private/var/tmp/_bazel_marvin/9cf0c27c2f651654c121c526ef57328f/external/crate_index__surrealdb-core-1.3.1/BUILD.bazel:16:13: 
no such target '@@crate_index__async-executor-1.8.0//:executor': target 'executor' not declared in package '' 
defined by /private/var/tmp/_bazel_marvin/9cf0c27c2f651654c121c526ef57328f/external/
crate_index__async-executor-1.8.0/BUILD.bazel 
and referenced by '@@crate_index__surrealdb-core-1.3.1//:surrealdb_core'
```

3) Query crate_index__async-executor-1.8.0

> bazel query "@crate_index__async-executor-1.8.0//:*"

Output:
```
@crate_index__async-executor-1.8.0//:.cargo_vcs_info.json
@crate_index__async-executor-1.8.0//:BUILD.bazel
@crate_index__async-executor-1.8.0//:CHANGELOG.md
@crate_index__async-executor-1.8.0//:Cargo.lock
@crate_index__async-executor-1.8.0//:Cargo.toml
@crate_index__async-executor-1.8.0//:Cargo.toml.orig
@crate_index__async-executor-1.8.0//:LICENSE-APACHE
@crate_index__async-executor-1.8.0//:LICENSE-MIT
@crate_index__async-executor-1.8.0//:README.md
@crate_index__async-executor-1.8.0//:async_executor
@crate_index__async-executor-1.8.0//:benches/executor.rs
@crate_index__async-executor-1.8.0//:examples/limit.rs
@crate_index__async-executor-1.8.0//:examples/priority.rs
@crate_index__async-executor-1.8.0//:src/lib.rs
@crate_index__async-executor-1.8.0//:tests/different_executors.rs
@crate_index__async-executor-1.8.0//:tests/drop.rs
@crate_index__async-executor-1.8.0//:tests/local_queue.rs
@crate_index__async-executor-1.8.0//:tests/panic_prop.rs

```

