mod db;
mod models;

use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use db::Database;
use models::{BuyPizzaRequest, Pizza, UpdatePizzaId};
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizza().await;

    match pizzas {
        Some(pizzas) => HttpResponse::Ok().json(pizzas),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/buy_pizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let name = body.name.clone();

            let pizza = db.buy_pizza(BuyPizzaRequest::create(name)).await;

            match pizza {
                Some(pizza) => HttpResponse::Ok().json(pizza),
                None => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Error: {}", e)),
    }
}

#[patch("/update_pizza/{id}")]
async fn update_pizza(id: Path<UpdatePizzaId>) -> impl Responder {
    let is_valid = id.into_inner().id;

    HttpResponse::Ok().body("Update pizza")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Failed to connect to database");

    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
