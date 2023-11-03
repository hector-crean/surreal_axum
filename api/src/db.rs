use models::credentials::Credentials;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Jwt, Root, Scope},
    Error, Surreal,
};

use surrealdb::sql::Geometry::Point;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub namespace: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        let jwt = client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client
            .use_ns("surreal")
            .use_db("conversations")
            .await
            .unwrap();

        let db = Database {
            client,
            namespace: String::from("surreal"),
            db_name: String::from("conversation"),
        };

        Ok(db)
    }

    pub async fn query_custom(self) -> Result<surrealdb::Response, surrealdb::Error> {
        let sql = "
            SELECT marketing, count() FROM type::table($table) GROUP BY marketing
        ";
        let resp = self.client.query(sql).bind(("table", "person")).await?;

        Ok(resp)
    }

    pub async fn sign_in<'a>(self, credentials: Credentials<'a>) -> Result<Jwt, surrealdb::Error> {
        let jwt = self
            .client
            .signin(Scope {
                namespace: "test",
                database: "test",
                scope: "user",
                params: credentials,
            })
            .await?;

        Ok(jwt)
    }
}
