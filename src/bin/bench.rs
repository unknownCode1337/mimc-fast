#![allow(non_snake_case)]

use itertools::iproduct;
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

    let (threshold, overflowed) = MimcState::rarity(PLANET_RARITY);
    assert!(!overflowed);

    let now = Instant::now();

    let _ = explorer
        .take(N)
        .par_bridge()
        .flat_map(|nextChunk| {
            let x = nextChunk.bottomLeft.x;
            let y = nextChunk.bottomLeft.y;
            let size = nextChunk.sideLength;

            iproduct!(x..(x + size), y..(y + size))
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
        })
        .collect::<Vec<Planet>>();

    // this.hashRate = chunk.chunkFootprint.sideLength ** 2 / (miningTimeMillis / 1000);
    let exp: f32 = (CHUNK_SIDE_LENGTH * CHUNK_SIDE_LENGTH).into();
    let elapsed = now.elapsed().as_millis() as f32 / 1000f32 / N as f32;
    println!("{:?}", (exp / elapsed));
}

#[derive(Clone, Debug, PartialEq)]
pub struct Planet2 {
    pub coords: Coords,
    pub hash: bigint::U512,
}
