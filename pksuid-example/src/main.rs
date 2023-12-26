use anyhow::Result;
use pksuid::Pksuid;
use sqlx::PgPool;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Client {
	id: Pksuid,
	name: String,
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
		select id as "id: Pksuid", name
		from client
		where id = $1;
	"#,
		inserted.id.clone() as Pksuid
	)
	.fetch_one(&pool)
	.await?;

	println!("selected: {:#?}", selected);

	assert_eq!(inserted.id, selected.id);

	Ok(())
}
