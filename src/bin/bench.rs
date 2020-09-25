#![allow(non_snake_case)]

use mimc_fast::explorers::*;
use mimc_fast::game::*;
use mimc_fast::mimc::*;
use rayon::prelude::*;
use std::time::Instant;

const PLANET_RARITY: u32 = 16384;
const N: usize = 100;
const CHUNK_SIDE_LENGTH: u16 = 16;
const CENTER: Coords = Coords { x: 0, y: 0 };

fn main() {
    let mut miner = SpiralMiner::new(CENTER, CHUNK_SIDE_LENGTH);

    // preallocate?
    let mut results: Vec<(Vec<Planet>, u32)> = vec![];
    let exp: f32 = (CHUNK_SIDE_LENGTH * CHUNK_SIDE_LENGTH).into();

    for _ in 0..N {
        let nextChunk = miner.next().unwrap();
        let now = Instant::now();
        let res = mine(nextChunk);

        // this.hashRate = chunk.chunkFootprint.sideLength ** 2 / (miningTimeMillis / 1000);
        let elapsed = now.elapsed().as_millis() as f32 / 1000f32;
        let rate = (exp / elapsed) as u32;

        let tup = (res, rate);
        results.push(tup);
    }

    let avg = results.iter().map(|res| res.1 as u64).sum::<u64>() / results.len() as u64;
    println!("{:?}", avg);
}

fn mine(chunkFootprint: ChunkFootprint) -> Vec<Planet> {
    let x = chunkFootprint.bottomLeft.x;
    let y = chunkFootprint.bottomLeft.y;
    let size = chunkFootprint.sideLength;

    let (threshold, overflowed) = MimcState::rarity(PLANET_RARITY);
    assert!(!overflowed);

    let planets = (x..(x + size))
        .into_par_iter()
        .map(|xi| {
            let mut planets = Vec::new();
            for yi in y..(y + size) {
                let hash = MimcState::sponge(vec![xi, yi], 1, 220)[0].x;
                if hash < threshold {
                    planets.push(Planet {
                        coords: Coords { x: xi, y: yi },
                        hash: hash.to_string(),
                    });
                }
            }
            planets
        })
        .flatten()
        .collect::<Vec<_>>();

    planets
}
