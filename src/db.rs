use sqlx::PgPool;
use url::Url;

use crate::model::Chart;

pub struct DB {
    pub pool: PgPool,
}

impl DB {
    pub async fn new() -> DB {
        let uri = DB::get_url();
        let pool = PgPool::connect(&uri).await.unwrap();
        DB { pool }
    }

    fn get_url() -> String {
        let host = dotenvy::var("DB_HOST").unwrap();
        let database = dotenvy::var("DB_NAME").unwrap();
        let user = dotenvy::var("DB_USER").unwrap();
        let pass = dotenvy::var("DB_PASS").unwrap();

        let db_uri = format!("postgres://{}:3306/{}", host, database);
        let mut uri = Url::parse(&db_uri).unwrap();
        uri.set_username(&user).expect("Failed to set username");
        uri.set_password(Some(&pass)).expect("Failed to set password");
        println!("DB url: {}", uri);
        uri.to_string()
    }

    pub async fn read_chart(&self) {
        let sql = "select id, chart_title, chart_tags from public.d_chart";
        println!("{}", sql);
        let rows = sqlx::query_as::<_, Chart>(&sql).fetch_all(&self.pool).await.unwrap();
        for row in rows {
            println!("{:#?}", row);
        }
    }
}