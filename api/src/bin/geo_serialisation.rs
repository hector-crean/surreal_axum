use geo::polygon;
use geo::Geometry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Company {
    id: String,
    home: Geometry,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let polygon = polygon![
      (x: -111., y: 45.),
      (x: -111., y: 41.),
      (x: -104., y: 41.),
      (x: -104., y: 45.),
    ];

    let company_1 = Company {
        id: "company:1".to_string(),
        home: polygon.into(),
    };

    println!("=========CASE 1==========================");
    println!("==========================================");
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;

    let results = db
        .query("INSERT INTO company $company;")
        .bind(("company", company_1));
    let mut results = results.await?;
    let complex_polygon_properly_deserialized: Option<Value> =
        results.take(0).expect("Should work");

    println!(
        "company: {}",
        serde_json::to_string(&complex_polygon_properly_deserialized).unwrap()
    );

    println!("=========CASE 2==========================");
    println!("==========================================");
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await?;
    let poly_complex = polygon!(
        exterior: [
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ],
        interiors: [
            [
                (x: -110., y: 44.),
                (x: -110., y: 42.),
                (x: -105., y: 42.),
                (x: -105., y: 44.),
            ],
        ],
    );
    let company_2 = Company {
        id: "company:2".to_string(),
        home: poly_complex.into(),
    };
    let results = db
        .query("INSERT INTO company $company;")
        .bind(("company", company_2));
    let mut results = results.await?;

    // The problematic line
    let complex_polygon_not_properly_deserialized: Option<Value> = results
        .take(0)
        .expect("transforms coordinates to strings rather f64 during deserialization");

    println!(
        "company: {}",
        serde_json::to_string(&complex_polygon_not_properly_deserialized).unwrap()
    );

    Ok(())
}
