use anyhow::Result;
use pksuid::Pksuid;
use sqlx::{PgPool, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
#[sqlx(type_name = "pksuid")]
struct SqlxPksuid(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Client {
	id: Pksuid,
	name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let pool = PgPool::connect(&database_url).await?;

	let inserted = sqlx::query_as!(
		Client,
		r#"
		insert into client (name)
		values ('Dave')
		returning id as "id: Pksuid", name;
	"#,
	)
	.fetch_one(&pool)
	.await?;

	println!("inserted: {:#?}", inserted);

	let selected = sqlx::query_as!(
		Client,
		r#"
		select name, id as "id: Pksuid"
		from client
		where id = $1;
	"#,
		inserted.id as Pksuid,
	)
	.fetch_one(&pool)
	.await?;

	println!("selected: {:#?}", selected);

	Ok(())
}
