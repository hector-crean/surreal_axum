use axum::Json;
use core::time::Duration;
use models::{
    credentials::Credentials,
    user::{CreateUser, User},
    video_label::{CreateVideoLabel, VideoLabel},
};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Jwt, Root, Scope},
    sql, Error, Surreal,
};
use tracing::info;

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

        client.use_ns("reg115").use_db("reg115").await.unwrap();

        let db = Database {
            client,
            namespace: String::from("reg115"),
            db_name: String::from("reg115"),
        };

        info!("Db initialised");

        Ok(db)
    }

    pub async fn create_user(
        self,
        Json(create_user_payload): Json<CreateUser>,
    ) -> Result<Vec<User>, surrealdb::Error> {
        // let sql = "
        //     SELECT marketing, count() FROM type::table($table) GROUP BY marketing
        // ";
        // let resp = self.client.query(sql).bind(("table", "person")).await?;

        let record: Vec<User> = self
            .client
            .create("user")
            .content(create_user_payload)
            .await?;

        Ok(record)
    }

    pub async fn get_users(self) -> Result<Vec<Body<User>>, surrealdb::Error> {
        let record: Vec<Body<User>> = self.client.select("user").await?;

        Ok(record)
    }

    pub async fn create_spacetime_geometry(
        self,
        Json(create_spacetime_geometry): Json<CreateSpacetimeGeometry>,
    ) -> Result<Vec<Body<SpacetimeGeometry>>, surrealdb::Error> {
        let CreateSpacetimeGeometry {
            author,
            geometry,
            timestamp,
            text_body,
            title,
            duration,
        } = create_spacetime_geometry;

        let sql = r#"
            BEGIN TRANSACTION;
            LET $spacetime_geometry = CREATE spacetime_geometry CONTENT {
                geometry: $geometry,
                author: $author,
                timestamp: $timestamp,
                title: $title,
                text_body: $text_body,
                duration: <duration> $duration
            };
            RETURN $spacetime_geometry;
            COMMIT TRANSACTION;
        "#;
        let mut resp = self
            .client
            .query(sql)
            .bind(("geometry", geometry))
            .bind(("author", author))
            .bind(("timestamp", timestamp))
            .bind(("title", title))
            .bind(("text_body", text_body))
            .bind(("duration", duration))
            .await?;

        let records = resp.take::<Vec<Body<SpacetimeGeometry>>>(0)?;

        // let records: Vec<Body<SpacetimeGeometry>> = self
        //     .client
        //     .create("spacetime_geometry")
        //     .content(create_spacetime_geometry)
        //     .await?;

        Ok(records)
    }

    pub async fn get_spacetime_geometries(
        self,
    ) -> Result<Vec<SpacetimeGeometry>, surrealdb::Error> {
        let records = self.client.select("spacetime_point").await?;

        Ok(records)
    }

    pub async fn signin<'a>(self, credentials: Credentials<'a>) -> Result<Jwt, surrealdb::Error> {
        let jwt = self
            .client
            .signin(Scope {
                namespace: self.namespace.as_str(),
                database: self.db_name.as_str(),
                scope: "user",
                params: credentials,
            })
            .await?;

        Ok(jwt)
    }

    pub async fn signup<'a>(self, credentials: Credentials<'a>) -> Result<Jwt, surrealdb::Error> {
        let jwt = self
            .client
            .signup(Scope {
                namespace: self.namespace.as_str(),
                database: self.db_name.as_str(),
                scope: "user",
                params: credentials,
            })
            .await?;

        Ok(jwt)
    }
}

#[cfg(test)]
mod tests {
    use geo_types::{point, polygon, Geometry, GeometryCollection};
    use geojson::Feature;

    use super::*;
    use crate::error;
    use serde::{Deserialize, Serialize};

    #[tokio::test]
    async fn test_signup_user() -> Result<(), error::ApiError> {
        let db = Database::init().await.expect("Database not started");

        let jwt = db
            .signup(Credentials {
                name: "Simon Harris",
                username: "simon harris",
                password: "123456789",
            })
            .await?;

        println!("{:?}", &jwt);

        Ok(())
    }

    #[tokio::test]
    async fn create_user_test() -> Result<(), error::ApiError> {
        let db = Database::init().await.expect("Database not started");

        let payload = CreateUser {
            name: String::from("Roby Crean"),
            username: String::from("robycrean"),
            password: String::from("password..."),
        };

        let resp = db.create_user(Json(payload)).await?;

        for r in resp {
            println!("{:?}", &r);
        }

        Ok(())
    }

    #[tokio::test]
    async fn create_spacetime_geometry() -> Result<(), error::ApiError> {
        let db = Database::init().await.expect("Database not started");

        let p: sql::Geometry = polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ]
        .into();

        let sql = r#"
            CREATE spacetime_geometry CONTENT {
               duration: <duration> "3w",
               geometry: $geometry,
               text_body: "text",
               title: "title",
               timestamp: 1.0,
            };"#;

        let mut query = db.client.query(sql).bind(("geometry", p));

        let mut resp = query.await?;

        let res = &resp.take::<Option<SpacetimeGeometry>>(0)?;

        println!("resp: {:?}", &res);

        // geometry<line|polygon|multipoint|multiline|multipolygon|collection>

        // This fails with the below error
        // let spacetime_geo: Option<SpacetimeGeometry> = response.take(0)?;

        // println!("{:?}", &spacetime_geo);
        // Fails just like the above
        // let users: Option<sql::Value> = response.take(0)?;

        // Works
        // let users: Option<serde_json::Value> = response.take(0)?;

        Ok(())
    }
}
