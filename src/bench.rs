use cramimc_fastte::mimc::*;
use mimc_fast::explorers::*;
use mimc_fast::game::*;
use rayon::prelude::*;

const planetRarity: u64 = 16384;

fn mine(chunkFootprint: ChunkFootprint) -> Vec<Planet> {
    let x = chunkFootprint.bottomLeft.x;
    let y = chunkFootprint.bottomLeft.y;
    let size = chunkFootprint.sideLength;

    let (threshold, overflowed) = MimcState::rarity(planetRarity);
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
                        hash,
                    });
                }
            }
            planets
        })
        .flatten()
        .collect::<Vec<_>>();

    planets
}

fn main() {
    let n = 1000;

    let results = vec![
        game::Planet {
            coords: Coords { x: 0, y: 0 },
            hash: 0.into()
        };
        n
    ];

    let center = Coords { x: 0, y: 0 };
    let chunkSideLength = 16;
    let mut miner = SpiralMiner::new(center, chunkSideLength);
    for i in 0..n {
        results.push(mine(miner.next()));
    }
}
