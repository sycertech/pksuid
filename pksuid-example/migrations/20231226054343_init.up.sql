-- Add up migration script here
create table if not exists client (
	id pksuid primary key default pksuid_generate('client'),
	name text not null
);
