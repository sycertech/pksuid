# pksuid

An extension for PostgreSQL providing a type and generation function for Prefixed KSUIDs, similar to that of [Clerk](https://clerk.com/blog/generating-sortable-stripe-like-ids-with-segment-ksuids) and [Stripe](https://www.quora.com/How-does-Stripe-generate-object-ids)

## Example

```sql
postgres=# create extension prefixed_ksuid;
create extension

postgres=# select pksuid_generate('client');
       pksuid_generate
------------------------------------
 client_2a40rvcCfXqllp2pWTNr6sH2wns

postgres=#
postgres=#
postgres=# create table if not exists client(
    -- todo: type `pksuid`
    id pksuid primary key default pksuid_generate('client'),
    name text
);
postgres=# insert into client(name) values('Dave');
INSERT 0 1
postgres=# select * from client;
                 id                 | name
------------------------------------+------
 client_2a48v6M9BKq9nBN5MkOREN1YTsl | Dave
(1 row)
```

## Roadmap

- [x] pgrx plugin
- [x] sqlx type
