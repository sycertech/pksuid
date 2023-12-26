# prefixed_ksuid_pgrx

> a plugin for postgresql providing a type and generation function for prefixed ksuids, similar to that of [Clerk](https://clerk.com/blog/generating-sortable-stripe-like-ids-with-segment-ksuids)

```sql
postgres=# create extension prefixed_ksuid;
create extension

postgres=# select prefixedksuid_generate('client');
       prefixedksuid_generate
------------------------------------
 client_2a40rvcCfXqllp2pWTNr6sH2wns

# TODO: this
postgres=# create table if not exists client(
    id prefixedksuid primary key default prefixedksuid_generate('client'),
    name text
);
ERROR:  data type prefixedksuid has no default operator class for access method "btree"
HINT:  You must specify an operator class for the index or define a default operator class for the data type.
```

## roadmap

- [x] pgrx plugin
- [x] sqlx type
