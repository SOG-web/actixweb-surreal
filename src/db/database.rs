use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::{BuyPizzaRequest, Pizza};

#[derive(Clone)]
pub struct Database {
    pub surreal: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client.use_ns("namespace").use_db("database").await.unwrap();

        Ok(Database {
            surreal: client,
            name_space: "tut".to_string(),
            db_name: "pizzas".to_string(),
        })
    }

    pub async fn get_all_pizza(&self) -> Option<Vec<Pizza>> {
        let result = self.surreal.select("pizza").await;

        match result {
            // Ok(pizzas) => {
            //     let mut pizza_list = Vec::new();
            //     for pizza in pizzas {
            //         let pizza = Pizza {
            //             id: pizza.get("id").unwrap().to_string(),
            //             name: pizza.get("name").unwrap().to_string(),
            //         };
            //         pizza_list.push(pizza);
            //     }
            //     Some(pizza_list)
            // }
            Ok(pizzas) => {
                // println!("Pizzas: {:?}", pizzas);
                Some(pizzas)
            }
            Err(e) => {
                println!("Error getting: {:?}", e);
                None
            }
        }
    }

    pub async fn buy_pizza(&self, pizza: BuyPizzaRequest) -> Option<Pizza> {
        let result = self.surreal.create("pizza").content(pizza).await;

        match result {
            Ok(pizza) => pizza,
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}
