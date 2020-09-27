//! cargo +nightly run --bin bench --release
#![allow(non_snake_case)]

use itertools::iproduct;
use mimc_fast::explorers::*;
use mimc_fast::game::*;
use mimc_fast::mimc::*;
use rayon::prelude::*;
use std::time::Instant;

const PLANET_RARITY: u32 = 16384;
const N: usize = 10;
const CENTER: Coords = Coords { x: 0, y: 0 };

fn main() {
    mine(16);
    mine(32);
    mine(64);
    mine(128);
    mine(256);
}

fn mine(chunkLength: u16) {
    let explorer = SpiralMiner::new(CENTER, chunkLength);

    let (threshold, overflowed) = MimcState::rarity(PLANET_RARITY);
    debug_assert!(!overflowed);

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
    let exp: f64 = (chunkLength * chunkLength).into();
    let elapsed = now.elapsed().as_millis() as f64 / 1000f64 / N as f64;
    println!("chunkLength {:?}: {:?}", chunkLength, (exp / elapsed));
}

#[derive(Clone, Debug, PartialEq)]
pub struct Planet2 {
    pub coords: Coords,
    pub hash: bigint::U512,
}
