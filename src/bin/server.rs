#![feature(proc_macro_hygiene, decl_macro)]
use itertools::iproduct;
use mimc_fast::game::*;
use mimc_fast::mimc::*;
use rayon::prelude::*;
use rocket::{get, http::Method, post, routes};
use rocket_contrib::json::Json;
use rocket_cors::{catch_all_options_routes, AllowedHeaders, AllowedOrigins};
use serde::{Deserialize, Serialize};

fn main() {
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();
    let options_routes = catch_all_options_routes();

    rocket::ignite()
        .mount("/", routes![mine, hello])
        .mount("/", options_routes)
        .manage(cors.clone())
        .attach(cors)
        .launch();
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/mine", data = "<task>")]
fn mine(task: Json<Task>) -> Json<Response> {
    let x = task.chunkFootprint.bottomLeft.x;
    let y = task.chunkFootprint.bottomLeft.y;
    let size = task.chunkFootprint.sideLength;

    let (threshold, overflowed) = MimcState::rarity(task.planetRarity);
    assert!(!overflowed);

    let planets = iproduct!(x..(x + size), y..(y + size))
        .par_bridge()
        .filter_map(|(xi, yi)| {
            let hash = MimcState::sponge(&[xi, yi], 220);
            if hash < threshold {
                Some(Planet {
                    coords: Coords { x: xi, y: yi },
                    hash: hash.to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<Planet>>();

    Json(Response {
        chunkFootprint: task.chunkFootprint.clone(),
        planetLocations: planets,
    })
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Task {
    chunkFootprint: ChunkFootprint,
    planetRarity: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Response {
    chunkFootprint: ChunkFootprint,
    planetLocations: Vec<Planet>,
}
