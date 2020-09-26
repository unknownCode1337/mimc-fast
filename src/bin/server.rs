#![feature(proc_macro_hygiene, decl_macro)]

// use log;
// use rayon::prelude::*;
use rocket::http::Method;
use rocket::{get, post, routes};
use rocket_contrib::json::Json;
use rocket_cors::{catch_all_options_routes, AllowedHeaders, AllowedOrigins};
use serde::{Deserialize, Serialize};

use mimc_fast::game::*;
use mimc_fast::mimc::*;

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

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/mine", data = "<task>")]
fn mine(task: Json<Task>) -> Json<Response> {
    // info_!("{:?}", task);
    // Info: Json(Task { chunkFootprint: ChunkFootprint { bottomLeft: Coords { x: -15456, y: 18944 }, sideLength: 32 }, planetRarity: 16384 })
    // Info: Json(Task { chunkFootprint: ChunkFootprint { bottomLeft: Coords { x: -15456, y: 18912 }, sideLength: 32 }, planetRarity: 16384 })
    // Info: Json(Task { chunkFootprint: ChunkFootprint { bottomLeft: Coords { x: -15456, y: 18880 }, sideLength: 32 }, planetRarity: 16384 })
    // Info: Json(Task { chunkFootprint: ChunkFootprint { bottomLeft: Coords { x: -15456, y: 18848 }, sideLength: 32 }, planetRarity: 16384 })

    let x = task.chunkFootprint.bottomLeft.x;
    let y = task.chunkFootprint.bottomLeft.y;
    let size = task.chunkFootprint.sideLength;

    let (threshold, overflowed) = MimcState::rarity(task.planetRarity);
    assert!(!overflowed);

    let xrange = x..(x + size);
    let yrange = y..(y + size);
    let planets = xrange
        .zip(yrange)
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
        .collect::<Vec<_>>();

    Json(Response {
        chunkFootprint: task.chunkFootprint.clone(),
        planetLocations: planets,
    })
}

fn main() {
    // for x in 0.. {
    //     if x % 100 == 0 {
    //         println!("trying ({}, 0)", x);
    //     }
    //     MimcState::sponge(vec![x, 0], 1, 220);
    // }

    // println!("{:?}", MimcState::sponge(vec![-2048, 0], 1, 220));
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

// private async nextValidExploreTarget(
//     chunkLocation: ChunkFootprint,
//     jobId: number
//   ): Promise<ChunkFootprint | null> {
//     // returns the first valid chunk equal to or after `chunk` (in the explore order of mining pattern) that hasn't been explored
//     // async because it may take indefinitely long to find the next target. this will block UI if done sync
//     // we use this trick to promisify:
//     // https://stackoverflow.com/questions/10344498/best-way-to-iterate-over-an-array-without-blocking-the-ui/10344560#10344560

//     // this function may return null if user chooses to stop exploring or changes mining pattern in the middle of its resolution
//     // so any function calling it should handle the null case appropriately
//     let candidateChunk = chunkLocation;
//     let count = 10000;
//     while (!this.isValidExploreTarget(candidateChunk) && count > 0) {
//       candidateChunk = this.miningPattern.nextChunk(candidateChunk);
//       count -= 1;
//     }
//     // since user might have switched jobs or stopped exploring during the above loop
//     if (!this.isExploring && jobId !== this.currentJobId) {
//       return null;
//     }
//     if (this.isValidExploreTarget(candidateChunk)) {
//       return candidateChunk;
//     }
//     return new Promise((resolve) => {
//       setTimeout(async () => {
//         const nextNextChunk = await this.nextValidExploreTarget(
//           candidateChunk,
//           jobId
//         );
//         resolve(nextNextChunk);
//       }, 0);
//     });
//   }

//   private exploreNext(fromChunk: ChunkFootprint, jobId: number) {
//     this.nextValidExploreTarget(fromChunk, jobId).then(
//       (nextChunk: ChunkFootprint | null) => {
//         if (!!nextChunk) {
//           const nextChunkKey = this.chunkLocationToKey(nextChunk, jobId);
//           this.exploringChunk[nextChunkKey] = {
//             chunkFootprint: nextChunk,
//             planetLocations: [],
//           };
//           this.exploringChunkStart[nextChunkKey] = Date.now();
//           this.minersComplete[nextChunkKey] = 0;
//           this.sendMessageToWorkers(nextChunk, jobId);
//         }
//       }
//     );
//   }
