# prefixed_ksuid_pgrx
> a plugin for postgresql providing a type and generation function for prefixed ksuids, similar to that of [Clerk](https://clerk.com/blog/generating-sortable-stripe-like-ids-with-segment-ksuids)

## roadmap
- [x] pgrx plugin
- [x] sqlx type

## test errors
```rust
PostgreSQL 16.1 on x86_64-pc-linux-gnu, compiled by gcc (GCC) 13.2.1 20231011 (Red Hat 13.2.1-4), 64-bit
[2023-12-25 17:23:52.942 MST] [267256] [658a1d18.413f8]: LOG:  starting PostgreSQL 16.1 on x86_64-pc-linux-gnu, compiled by gcc (GCC) 13.2.1 20231011 (Red Hat 13.2.1-4), 64-bit
[2023-12-25 17:23:52.943 MST] [267256] [658a1d18.413f8]: LOG:  listening on IPv4 address "127.0.0.1", port 32216
[2023-12-25 17:23:52.949 MST] [267256] [658a1d18.413f8]: LOG:  listening on Unix socket "/home/carter/.pgrx/.s.PGSQL.32216"
[2023-12-25 17:23:52.957 MST] [267259] [658a1d18.413fb]: LOG:  database system was shut down at 2023-12-25 15:20:38 MST
     Creating database pgrx_tests
thread 'tests::pg_test_prefixed_ksuid_input' panicked at /home/carter/.cargo/registry/src/index.crates.io-6f17d22bba15001f/pgrx-tests-0.11.2/src/framework.rs:172:9:


Postgres Messages:
[2023-12-25 17:23:52.942 MST] [267256] [658a1d18.413f8]: LOG:  starting PostgreSQL 16.1 on x86_64-pc-linux-gnu, compiled by gcc (GCC) 13.2.1 20231011 (Red Hat 13.2.1-4), 64-bit
[2023-12-25 17:23:52.943 MST] [267256] [658a1d18.413f8]: LOG:  listening on IPv4 address "127.0.0.1", port 32216
[2023-12-25 17:23:52.949 MST] [267256] [658a1d18.413f8]: LOG:  listening on Unix socket "/home/carter/.pgrx/.s.PGSQL.32216"
[2023-12-25 17:23:52.981 MST] [267256] [658a1d18.413f8]: LOG:  database system is ready to accept connections


Test Function Messages:
[2023-12-25 17:23:53.110 MST] [267290] [658a1d19.4141a]: LOG:  statement: BEGIN
[2023-12-25 17:23:53.110 MST] [267290] [658a1d19.4141a]: LOG:  statement: SELECT "tests"."test_prefixed_ksuid_input"();
[2023-12-25 17:23:53.111 MST] [267290] [658a1d19.4141a]: ERROR:  type "prefixedksuid" does not exist
[2023-12-25 17:23:53.111 MST] [267290] [658a1d19.4141a]: STATEMENT:  SELECT "tests"."test_prefixed_ksuid_input"();
[2023-12-25 17:23:53.111 MST] [267290] [658a1d19.4141a]: LOG:  statement: ROLLBACK


Client Error:
type "prefixedksuid" does not exist
postgres location: parse_type.c
rust location: <unknown>


note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- tests::pg_test_prefixed_ksuid_create stdout ----
thread 'tests::pg_test_prefixed_ksuid_create' panicked at /home/carter/.cargo/registry/src/index.crates.io-6f17d22bba15001f/pgrx-tests-0.11.2/src/framework.rs:172:9:


Postgres Messages:
[2023-12-25 17:23:52.942 MST] [267256] [658a1d18.413f8]: LOG:  starting PostgreSQL 16.1 on x86_64-pc-linux-gnu, compiled by gcc (GCC) 13.2.1 20231011 (Red Hat 13.2.1-4), 64-bit
[2023-12-25 17:23:52.943 MST] [267256] [658a1d18.413f8]: LOG:  listening on IPv4 address "127.0.0.1", port 32216
[2023-12-25 17:23:52.949 MST] [267256] [658a1d18.413f8]: LOG:  listening on Unix socket "/home/carter/.pgrx/.s.PGSQL.32216"
[2023-12-25 17:23:52.981 MST] [267256] [658a1d18.413f8]: LOG:  database system is ready to accept connections


Test Function Messages:
[2023-12-25 17:23:53.110 MST] [267289] [658a1d19.41419]: LOG:  statement: BEGIN
[2023-12-25 17:23:53.110 MST] [267289] [658a1d19.41419]: LOG:  statement: SELECT "tests"."test_prefixed_ksuid_create"();
[2023-12-25 17:23:53.111 MST] [267289] [658a1d19.41419]: ERROR:  type "prefixedksuid" does not exist
[2023-12-25 17:23:53.111 MST] [267289] [658a1d19.41419]: STATEMENT:  SELECT "tests"."test_prefixed_ksuid_create"();
[2023-12-25 17:23:53.111 MST] [267289] [658a1d19.41419]: LOG:  statement: ROLLBACK


Client Error:
type "prefixedksuid" does not exist
postgres location: parse_type.c
rust location: <unknown>



---- tests::pg_test_prefixed_ksuid_output stdout ----
thread 'tests::pg_test_prefixed_ksuid_output' panicked at /home/carter/.cargo/registry/src/index.crates.io-6f17d22bba15001f/pgrx-tests-0.11.2/src/framework.rs:172:9:


Postgres Messages:
[2023-12-25 17:23:52.942 MST] [267256] [658a1d18.413f8]: LOG:  starting PostgreSQL 16.1 on x86_64-pc-linux-gnu, compiled by gcc (GCC) 13.2.1 20231011 (Red Hat 13.2.1-4), 64-bit
[2023-12-25 17:23:52.943 MST] [267256] [658a1d18.413f8]: LOG:  listening on IPv4 address "127.0.0.1", port 32216
[2023-12-25 17:23:52.949 MST] [267256] [658a1d18.413f8]: LOG:  listening on Unix socket "/home/carter/.pgrx/.s.PGSQL.32216"
[2023-12-25 17:23:52.981 MST] [267256] [658a1d18.413f8]: LOG:  database system is ready to accept connections


Test Function Messages:
[2023-12-25 17:23:53.111 MST] [267292] [658a1d19.4141c]: LOG:  statement: BEGIN
[2023-12-25 17:23:53.111 MST] [267292] [658a1d19.4141c]: LOG:  statement: SELECT "tests"."test_prefixed_ksuid_output"();
[2023-12-25 17:23:53.112 MST] [267292] [658a1d19.4141c]: ERROR:  Datum error: Postgres type prefixed_ksuid oid=#16409 is not compatible with the Rust type alloc::string::String oid={#25, builtin: TEXTOID}
[2023-12-25 17:23:53.112 MST] [267292] [658a1d19.4141c]: STATEMENT:  SELECT "tests"."test_prefixed_ksuid_output"();
[2023-12-25 17:23:53.112 MST] [267292] [658a1d19.4141c]: LOG:  statement: ROLLBACK


Client Error:
Datum error: Postgres type prefixed_ksuid oid=#16409 is not compatible with the Rust type alloc::string::String oid={#25, builtin: TEXTOID}
postgres location: panic.rs
rust location: <unknown>




failures:
    tests::pg_test_prefixed_ksuid_create
    tests::pg_test_prefixed_ksuid_input
    tests::pg_test_prefixed_ksuid_output

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.83s

stopping postgres (pid=267256)
error: test failed, to rerun pass `--lib`
```
