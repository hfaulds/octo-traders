// https://github.com/Lapz/perlin_noise/blob/master/src/perlin.rs
// Needs rand with wasm-bindgen

use rand::{thread_rng,Rng};

/// Perlin Noise generator that outputs 1/2/3D Perlin noise
#[derive(Clone)]
pub struct PerlinNoise {
    perm: [usize; 512],
    octaves: usize,
    fallout: f64,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut perm = [0; 512];
        let mut rng = thread_rng();

        for i in 0..256 {
            perm[i] = i;
        }

        for i in 0..256 {
            let j = rng.gen_range(0, 256) & 0xFF;
            let t = perm[j];

            perm[j] = perm[i];
            perm[i] = t;
        }

        for i in 0..256 {
            perm[i + 256] = perm[i];
        }

        PerlinNoise {
            perm,
            octaves: 4,
            fallout: 0.5,
        }
    }

    /// Perlin Noise in 2D
    pub fn get2d(&self, args: [f64; 2]) -> f64 {
        let mut effect = 1.0;
        let mut k = 1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * ((1.0 + self.noise2d(k * args[0], k * args[1])) / 2.0);

            k *= 2.0
        }

        sum
    }

    fn noise2d(&self, mut x: f64, mut y: f64) -> f64 {
        let x0 = (x.floor() as usize) & 255;
        let y0 = (y.floor() as usize) & 255;

        x -= x.floor();
        y -= y.floor();

        let fx = (3.0 - 2.0 * x) * x * x;
        let fy = (3.0 - 2.0 * y) * y * y;
        let p0 = self.perm[x0] + y0;
        let p1 = self.perm[x0 + 1] + y0;

        lerp(
            fy,
            lerp(
                fx,
                grad2d(self.perm[p0], x, y),
                grad2d(self.perm[p1], x - 1.0, y),
            ),
            lerp(
                fx,
                grad2d(self.perm[p0 + 1], x, y - 1.0),
                grad2d(self.perm[p1 + 1], x - 1.0, y - 1.0),
            ),
        )
    }
}

fn grad2d(hash: usize, x: f64, y: f64) -> f64 {
    let v = if hash & 1 == 0 { x } else { y };

    if (hash & 1) == 0 {
        -v
    } else {
        v
    }
}

// Linear Interpolate
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}
// Fade function as defined by Ken Perlin.  This eases coordinate values
// so that they will "ease" towards integral values.  This ends up smoothing
// the final output.
