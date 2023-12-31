mod model;
mod schema;
use self::model::*;
use self::schema::cats::dsl::*;
use actix_web::{web, App, HttpServer, HttpResponse, Result, Error, error};
use actix_files::{Files, NamedFile};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::collections::HashMap;
use serde::Deserialize;
use validator::Validate;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[derive(Deserialize, Validate)]
struct CatEndpointPath {
    #[validate(range(min = 1, max = 150))]
    id: i32,
}

async fn add_cat_endpoint(
    pool:web::Data<DbPool>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse, Error> {
    let file_path = parts
        .files
        .take("image")
        .pop()
        .and_then(|f| f.persist_in("./image").ok())
        .unwrap_or_default();

    let text_fields: HashMap<_,_> = parts
        .texts
        .as_pairs()
        .into_iter()
        .collect();

    let mut connection = pool.get()
        .expect("Can't get db connection from pool");

    let new_cat = NewCat {
        name: text_fields.get("name").unwrap().to_string(),
        image_path: file_path.to_string_lossy().to_string()
    };

    web::block(move ||
        diesel::insert_into(cats)
        .values(&new_cat)
        .execute(&mut connection)
        )
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().finish())

}

async fn cats_endpoint(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut connection =
        pool.get().expect("Can't get db connection from pool");

    let cats_data = web::block(move ||{
        cats.limit(100).load::<Cat>(&mut connection)
    })
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cats_data))
}

async fn cat_endpoint(
    pool: web::Data<DbPool>,
    cat_id: web::Path<CatEndpointPath>,
) -> Result<HttpResponse, Error> {
    cat_id
        .validate()
        .map_err(error::ErrorBadRequest)?;

    let mut connection =
        pool.get().expect("Can't get db connection from pool");

    let cats_data = web::block(move ||{
        cats.filter(id.eq(cat_id.id))
            .first::<Cat>(&mut connection)
    })
        .await?.map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(cats_data))
}

fn setup_database() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager =
        ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.")
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/cats", web::get().to(cats_endpoint))
            .route("/add_cat", web::post().to(add_cat_endpoint))
            .route("/cat/{id}", web::get().to(cat_endpoint))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = setup_database();

    println!("Listening on port 8080");
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(
                awmp::PartsConfig::default()
                .with_temp_dir("./tmp")
            )
            .configure(api_config) // Used here
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .service(
                Files::new("/image","image")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_cats_endpoint_get() {
        let pool = setup_database();
        let mut app = test::init_service(
            App::new().app_data(web::Data::new(pool.clone()))
                .configure(api_config),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/cats")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
