// External imports
use actix_cors::Cors;
use actix_web::{http, middleware};
use dotenv::dotenv;
use std::error::Error;


// don't forget this!
//use tokio::stream::StreamExt;
use std::{env, io};
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use mongodb::{options::ClientOptions, bson::Document, bson::doc, options::FindOptions};

#[allow(unused_imports)]
use futures::stream::StreamExt;
#[allow(unused_imports)]
use std::sync::*;

#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().json("Hola Mundo desde RUST")
}

#[allow(non_snake_case)]
#[get("/getData")]
async fn get_data() -> impl Responder {
    let database_url = "mongodb://mongoadmin:Arqui2022_2022@localhost:27017";
    let mut client_options = ClientOptions::parse(&database_url).await.unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database(&env::var("DATABASE_NAME").unwrap());
    let collection = db.collection::<Document>(&env::var("USER_COLLECTION_NAME").unwrap());

    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}
#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv::dotenv().ok();
    let port = env::var("PORT").unwrap();
    let address = format!("{}:{}", "0.0.0.0", "2000");
    println!("Rust server on port {} :D", port);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_data)
            .service(test)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("Couldn't start the server in port {}: {:?} :O",port, err )
    })
    .run()
    .await
}