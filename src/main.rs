mod bitmaps;
use core::time;
use std::thread::sleep;
use bitmaps::bitmaps::{get_height, get_map, get_width};
use minifb::{Key, Window, WindowOptions};
const WIDTH: usize = 640;
const HEIGHT: usize = 360;
#[derive(Clone, Copy)]
pub enum BlockType {
    Straight = 1,
    Square,
    Tee,
    Ell,
    Skew,
}
#[derive(Clone)]
struct Block {
    direction: i8,
    block_type: BlockType,
    width : u8,
    height : u8,
    pixels : Vec<u32>
}
impl Block {
    fn new(block: BlockType) -> Self {
        Block {
            direction: 1,
            block_type: block,
            width : get_width(block),
            height : get_height(block),
            pixels : get_map(block)
        }
    }
}
fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let time = time::Duration::from_secs(1);
    let mut window = Window::new(
        "Tetris - in Rust",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);
    let block = Block::new(BlockType::Straight);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for j in (0..400).step_by(10) {
            // create block
            if window.is_open() && !window.is_key_down(Key::Escape) {
                // maybe try another method for delay
                sleep(time);
                for i in 0..block.pixels.len() {
                    // divide and mod is for the width value of the block
                    let c = ((50 + j + (i / block.width as usize)) * 640) + 40 + (i % block.width as usize);
                    if c >= buffer.len() {
                        break;
                    }
                    buffer[c] = block.pixels[i];
                }
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                // clear the block
                for i in 0..block.pixels.len() {
                    // divide and mod is for the width value of the block
                    let c = ((50 + j + (i / block.width as usize)) * 640) + 40 + (i % block.width as usize);
                    if c >= buffer.len() {
                        break;
                    }
                    buffer[c] = 0;
                }
            }
        }
    }
}
