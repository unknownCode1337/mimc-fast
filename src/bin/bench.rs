/// This runs non parallel as with current design cant flatten and collect
/// also hash calc is broken?
#![allow(non_snake_case)]

use mimc_fast::explorers::*;
use mimc_fast::game::*;
use mimc_fast::mimc::*;
use rayon::prelude::*;
use std::time::Instant;

const PLANET_RARITY: u32 = 16384;
const N: usize = 10;
const CHUNK_SIDE_LENGTH: u16 = 16;
const CENTER: Coords = Coords { x: 0, y: 0 };

fn main() {
    let explorer = SpiralMiner::new(CENTER, CHUNK_SIDE_LENGTH);
    let exp: f32 = (CHUNK_SIDE_LENGTH * CHUNK_SIDE_LENGTH).into();

    let (threshold, overflowed) = MimcState::rarity(PLANET_RARITY);
    assert!(!overflowed);

    let now = Instant::now();

    let planets = explorer
        .take(N)
        .map(|nextChunk| {
            let x = nextChunk.bottomLeft.x;
            let y = nextChunk.bottomLeft.y;
            let size = nextChunk.sideLength;

            let range: Vec<(i64, i64)> = (x..(x + size))
                .map(|xi| (y..(y + size)).map(move |yi| (xi, yi)))
                .flatten()
                .collect();

            range.iter().filter_map(|(xi, yi)| {
                let hash = MimcState::sponge(&[xi, yi], 220);
                if hash < threshold {
                    Some(Planet2 {
                        coords: Coords { x: xi, y: yi },
                        hash,
                    })
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<Planet2>>();

    // this.hashRate = chunk.chunkFootprint.sideLength ** 2 / (miningTimeMillis / 1000);
    let elapsed = now.elapsed().as_millis() as f32 / 1000f32 / N as f32;
    let rate = (exp / elapsed) as u32;

    // let avg = results.iter().map(|res| res.1 as u64).sum::<u64>() / results.len() as u64;
    // let avg = results.iter().cloned().sum::<u32>() / results.len() as u32;

    println!("{:?}", rate);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Planet2 {
    pub coords: Coords,
    pub hash: bigint::U512,
}
