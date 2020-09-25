#![allow(non_snake_case)]

use mimc_fast::explorers::*;
use mimc_fast::game::*;
use mimc_fast::mimc::*;
use rayon::prelude::*;
use std::time::Instant;

const PLANET_RARITY: u32 = 16384;

fn main() {
    const N: usize = 100;

    let center = Coords { x: 0, y: 0 };
    let chunkSideLength: u16 = 16;

    let mut miner = SpiralMiner::new(center, chunkSideLength);

    // preallocate?
    let mut results: Vec<(Vec<Planet>, u64)> = vec![];
    let exp: u64 = (chunkSideLength * chunkSideLength).into();
    for _ in 0..N {
        let nextChunk = miner.next().unwrap();
        let now = Instant::now();
        let res = mine(nextChunk);

        // this.hashRate = chunk.chunkFootprint.sideLength ** 2 / (miningTimeMillis / 1000);
        let rate = exp / (now.elapsed().as_millis() as u64 / 1000u64);

        let tup = (res, rate);
        results.push(tup);
    }

    print!("{:?}", results);
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
