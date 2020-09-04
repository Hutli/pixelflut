use crate::alphabet::char_to_pixel_positions;
use io::Write;
use numtoa::NumToA;
use std::{io, net::TcpStream};

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;
struct Pixel<'t> {
    x: u32,
    y: u32,
    color: &'t [u8],
}

const SCREEN_DIM_MAX_LEN: usize = 4;
const COMMAND_SIZE: usize = 19; // "PX XXXX YYY CCCCCC\n"

pub struct Packet {
    data: Vec<u8>,
    data_i: usize,
    xoffset: u32,
    yoffset: u32,
    scale: u32,
    //buf: [u8; SCREEN_DIM_MAX_LEN],
}

impl Packet {
    pub fn new(scale: u32) -> Packet {
        Packet {
            data: vec![0; COMMAND_SIZE * SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize],
            data_i: 0,
            xoffset: 0,
            yoffset: 0,
            scale,
            //buf: [0 as u8; SCREEN_DIM_MAX_LEN],
        }
    }

    fn write_and_update_index(&mut self, slice: &[u8]) {
        let len = slice.len();
        self.data[self.data_i..self.data_i + len].copy_from_slice(slice);
        self.data_i += len;
    }

    //#[inline(never)]
    fn add_pixel(&mut self, pixel: Pixel) {
        self.write_and_update_index(b"PX ");
        let tmp = pixel.x.numtoa(10, &mut self.data[self.data_i..]);
        //let tmp = self.lookup(pixel.x);
        self.data_i += tmp.len();

        self.write_and_update_index(b" ");
        let tmp = pixel.y.numtoa(10, &mut self.data[self.data_i..]);
        //let tmp = self.lookup(pixel.y);
        self.data_i += tmp.len();
        self.write_and_update_index(b" ");
        self.write_and_update_index(pixel.color);
        self.write_and_update_index(b"\n");
    }

    pub fn add_letter(&mut self, c: char) {
        let scale_y: u32 = self.scale;
        let scale_x: u32 = self.scale;
        let width: u32 = 8;
        let height: u32 = 8;
        // let array = &self.alphabet[0];
        let array = char_to_pixel_positions(c).expect(&format!("Invalid char: {}", c));
        for y in 0..height {
            //print!("{}\n", termion::style::Reset);
            for x in 0..width {
                let i = x + y * width;
                let array_contains_i = array.contains(&i);
                for zy in 0..scale_y {
                    for zx in 0..scale_x {
                        let color: &mut [u8] = &mut ['0' as u8; 6];
                        let x = zx + self.xoffset + x * scale_x;
                        let y = zy + self.yoffset + y * scale_y;
                        if !array_contains_i {
                            rand::random::<u8>().numtoa(16, &mut color[0..2]);
                            rand::random::<u8>().numtoa(16, &mut color[2..4]);
                            rand::random::<u8>().numtoa(16, &mut color[4..6]);
                        }
                        let pixel = Pixel { x, y, color };
                        self.add_pixel(pixel)
                        //print!(" ");
                    }
                }
            }
            //print!("{}\n", termion::style::Reset)
        }
        self.xoffset += width * self.scale;
        if self.xoffset + width * self.scale > SCREEN_WIDTH {
            self.xoffset = 0;
            self.yoffset += height * self.scale;
        }
    }

    pub fn add_string(&mut self, s: &str) {
        for c in s.chars() {
            self.add_letter(c);
        }
    }

    pub fn write(&mut self, stream: &mut TcpStream) -> io::Result<usize> {
        stream.write(&self.data[0..self.data_i])
    }

    pub fn reset(&mut self) {
        self.xoffset = 0;
        self.yoffset = 0;
        self.data_i = 0;
    }
}