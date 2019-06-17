# Rust based postgres extension

Example functions for testing pg-extend-rs logging.

To build, get Rust, then:

```console
$> cargo build --release
...
```

then load into postgres

```console
$> psql $CONN_STR
postgres=# CREATE FUNCTION rs_nullif(text,text) RETURNS text AS 'path/to/libnullable.dylib', 'pg_rs_nullif' LANGUAGE C;
```
