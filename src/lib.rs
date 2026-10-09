use clap::ValueEnum;
use image::{Rgb, RgbImage, imageops};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Pattern {
    Symmetrical,
    Asymmetrical,
}

pub struct Identicon {
    height: u32,
    width: u32,
    block_height: u32,
    block_width: u32,
}

impl Identicon {
    pub fn new(height: u32, width: u32, block_height: u32, block_width: u32) -> Identicon {
        Identicon {
            height,
            width,
            block_height,
            block_width,
        }
    }

    pub fn calculate_hash(&self, unique_id: &str) -> Vec<u8> {
        // Sha256 produces 32 bytes
        // TODO: How to handle block size exceeding 32 bytes?
        Sha256::digest(unique_id.as_bytes()).to_vec()
    }

    pub fn generate_pattern(&self, hash: &[u8], pattern: Pattern) -> Vec<bool> {
        // TODO: How to handle non square dimensions? (Done: image resize takes care of this)
        // TODO: How to handle block not divisible evenly by image?

        // Two designs
        match pattern {
            // 1. Github blog: Symmmetrical horizontally so we care about only half of the image rest is mirror flip
            Pattern::Symmetrical => {
                let blocks_per_row = self.block_width.div_ceil(2);
                let block_cells = blocks_per_row * self.block_height;

                assert!(self.block_width > 0);
                assert!(hash.len() >= block_cells as usize);

                let block_grid: Vec<bool> = hash
                    .iter()
                    .take(block_cells as usize)
                    .map(|&byte| byte % 2 == 0)
                    .collect();
                // Select chunks per row and flip from mid and append
                let mirrored_grid: Vec<bool> = block_grid
                    .chunks(blocks_per_row as usize)
                    .flat_map(|row| {
                        row.iter()
                            .chain(row.iter().rev().skip((self.block_width % 2) as usize))
                            .copied()
                    })
                    .collect();
                mirrored_grid
            }
            // // 2. Not symemerical design
            Pattern::Asymmetrical => {
                let block_cells = self.block_height * self.block_width;
                // Traverse through all rows and in each row flip to 1 if hash byte corresponding to that index is even
                let block_grid: Vec<bool> = hash
                    .iter()
                    .take(block_cells as usize)
                    .map(|&byte| byte % 2 == 0)
                    .collect();
                block_grid
            }
        }
    }

    pub fn get_random_color(&self, hash: &[u8]) -> (u8, u8, u8) {
        // Use last 32 - (5 * 5) bytes to select a random color
        // Use last 3 bytes as R G B components
        let red_value = hash[hash.len() - 3];
        let green_value = hash[hash.len() - 2];
        let blue_value = hash[hash.len() - 1];
        (red_value, green_value, blue_value)
    }

    pub fn render_pattern(&self, rgb_color: &(u8, u8, u8), block_grid: &[bool], out_path: &str) {
        // Convert block pattern to image pattern
        // each value from block grid corresponds to 5 x 5 block in image
        let mut block_img = RgbImage::new(self.block_width, self.block_height);
        block_img
            .enumerate_pixels_mut()
            .for_each(|(x, y, pixel_value)| {
                let index = (y * self.block_width + x) as usize;

                *pixel_value = if block_grid[index] {
                    Rgb([rgb_color.0, rgb_color.1, rgb_color.2]) // fill with selected color
                } else {
                    Rgb([255, 255, 255]) // fill with white
                };
            });
        let final_img = imageops::resize(
            &block_img,
            self.width,
            self.height,
            imageops::FilterType::Nearest,
        );
        let _ = final_img.save(out_path);
    }
}
