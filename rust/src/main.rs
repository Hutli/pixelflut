use rand::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::vec::Vec;
// use std::str::from_utf8;
// use termion::raw::IntoRawMode;
//use termion::input::TermRead;

const SERVER_IP: &str = "192.168.0.96:1337";
//const SERVER_IP:&str = "192.168.0.161:1337";
const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 560;
// https://i0.wp.com/luckyresistor.me/wp-content/uploads/2015/09/font1.png?ssl=1

struct Pixel {
    x: i32,
    y: i32,
    color: String,
}
const MAX_COMMAND_SIZE: usize = 19; // "PX XXXX YYY CCCCCC\n"

fn print_letter(
    alphabet: &Vec<Vec<i32>>,
    character: usize,
    xoffset: i32,
    yoffset: i32,
    scale: i32,
) -> &'static[u8] {
    let zoom_y: i32 = scale;
    let zoom_x: i32 = scale;
    let width: i32 = 8;
    let height: i32 = 8;
    let array = &alphabet[character];
    let letter = &mut vec![];
    for y in 0..height {
        //print!("{}\n", termion::style::Reset);
        for x in 0..width {
            let i = x + y * width;
            for zy in 0..zoom_y {
                for zx in 0..zoom_x {
                    if array.contains(&i) {
                        //print!("|{}|", color);
                        let pixel = Pixel{
                            x: zx + xoffset + x * zoom_x,
                            y: zy + yoffset + y * zoom_y,
                            color: "000000".to_string()};
                        letter.append(&mut write_pixel(pixel).to_vec());
                    //print!("{} ", termion::color::Bg(termion::color::Red));
                    } else {
                        let r = format!("{:02x?}", rand::random::<i8>());
                        let g = format!("{:02x?}", rand::random::<i8>());
                        let b = format!("{:02x?}", rand::random::<i8>());
                        let color = format!("{}{}{}", r, g, b);
                        let pixel = Pixel{
                            x: zx + xoffset + x * zoom_x,
                            y: zy + yoffset + y * zoom_y,
                            color: color};
                        letter.append(&mut write_pixel(pixel).to_vec());
                        //print!("{} ", termion::color::Bg(termion::color::Blue));
                    }
                }
            }
        }
    }
    return letter;
}

// fn write_all(pixels:vec![Pixel]){
//     let package:vec![u8] = b'';
//     for pixel in pixels {
//         msg = format_pixelflut_package(pixel->x, pixel->y, pixel->color);
//         package
//     }
// }

// fn format_pixelflut_package(x:i32, y:i32, color:String) -> [u8]{
//     let msg = format!("PX {} {} {}\n", x.to_string(), y.to_string(), color);
//     return msg.as_bytes();
// }

fn write_pixel(pixel:Pixel) -> &'static[u8] {
    let msg = format!("PX {} {} {}\n", (pixel.x).to_string(), (pixel.y).to_string(), pixel.color);
    return msg.as_bytes();
}

fn main() {
    let mut stream = TcpStream::connect(SERVER_IP).expect("Failed to connect");
    println!("Successfully connected to server {}", SERVER_IP);
    loop {
        let alphabet:Vec<Vec<i32>> = vec![vec![],vec![3,4,11,12,19,20,27,28,35,36,51,52],vec![1,2,5,6,9,10,13,14,17,18,21,22],vec![1,2,5,6,9,10,13,14,16,17,18,19,20,21,22,23,25,26,29,30,32,33,34,35,36,37,38,39,41,42,45,46,49,50,53,54],vec![3,4,10,11,12,13,14,17,18,26,27,28,29,37,38,41,42,43,44,45,51,52],vec![1,2,5,6,9,10,13,14,20,21,27,28,34,35,41,42,45,46,49,50,53,54],vec![2,3,4,9,10,12,13,17,18,20,21,26,27,28,30,31,33,34,36,37,38,41,42,45,46,50,51,52,54,55],vec![3,4,11,12,19,20],vec![4,5,11,12,18,19,26,27,34,35,43,44,52,53],vec![2,3,11,12,20,21,28,29,36,37,43,44,50,51],vec![9,10,13,14,18,19,20,21,24,25,26,27,28,29,30,31,34,35,36,37,41,42,45,46],vec![11,12,19,20,25,26,27,28,29,30,35,36,43,44],vec![43,44,51,52,58,59],vec![25,26,27,28,29,30],vec![43,44,51,52],vec![6,7,13,14,20,21,27,28,34,35,41,42,48,49],vec![2,3,4,5,9,10,13,14,17,18,20,21,22,25,26,27,29,30,33,34,37,38,41,42,45,46,50,51,52,53],vec![3,4,10,11,12,19,20,27,28,35,36,43,44,50,51,52,53],vec![2,3,4,5,9,10,13,14,21,22,27,28,29,34,35,41,42,49,50,51,52,53,54],vec![2,3,4,5,9,10,13,14,21,22,28,29,37,38,41,42,45,46,50,51,52,53],vec![4,5,6,11,12,13,14,18,19,21,22,25,26,29,30,33,34,35,36,37,38,39,45,46,53,54],vec![1,2,3,4,5,6,9,10,17,18,19,20,21,29,30,37,38,41,42,45,46,50,51,52,53],vec![2,3,4,5,9,10,13,14,17,18,25,26,27,28,29,33,34,37,38,41,42,45,46,50,51,52,53],vec![1,2,3,4,5,6,9,10,13,14,21,22,28,29,35,36,43,44,51,52],vec![2,3,4,5,9,10,13,14,17,18,21,22,26,27,28,29,33,34,37,38,41,42,45,46,50,51,52,53],vec![2,3,4,5,9,10,13,14,17,18,21,22,26,27,28,29,30,37,38,41,42,45,46,50,51,52,53],vec![11,12,19,20,43,44,51,52],vec![11,12,19,20,43,44,51,52,58,59],vec![4,5,11,12,18,19,25,26,34,35,43,44,52,53],vec![17,18,19,20,21,22,33,34,35,36,37,38],vec![1,2,10,11,19,20,28,29,35,36,42,43,49,50],vec![2,3,4,5,9,10,13,14,21,22,28,29,35,36,51,52],vec![2,3,4,5,9,10,13,14,17,18,20,21,22,25,26,28,29,30,33,34,41,42,46,47,50,51,52,53,54],vec![2,3,4,5,9,10,13,14,17,18,21,22,25,26,27,28,29,30,33,34,37,38,41,42,45,46,49,50,53,54],vec![1,2,3,4,5,9,10,13,14,17,18,21,22,25,26,27,28,29,33,34,37,38,41,42,45,46,49,50,51,52,53],vec![2,3,4,5,9,10,13,14,17,18,25,26,33,34,41,42,45,46,50,51,52,53],vec![1,2,3,4,5,9,10,13,14,17,18,21,22,25,26,29,30,33,34,37,38,41,42,45,46,49,50,51,52,53],vec![1,2,3,4,5,6,9,10,17,18,25,26,27,28,33,34,41,42,49,50,51,52,53,54],vec![1,2,3,4,5,6,9,10,17,18,25,26,27,28,33,34,41,42,49,50],vec![2,3,4,5,9,10,13,14,17,18,25,26,28,29,30,33,34,37,38,41,42,45,46,50,51,52,53,54],vec![1,2,5,6,9,10,13,14,17,18,21,22,25,26,27,28,29,30,33,34,37,38,41,42,45,46,49,50,53,54],vec![2,3,4,5,11,12,19,20,27,28,35,36,43,44,50,51,52,53],vec![5,6,13,14,21,22,29,30,33,34,37,38,41,42,45,46,50,51,52,53],vec![1,2,5,6,9,10,13,14,17,18,20,21,25,26,27,28,33,34,36,37,41,42,45,46,49,50,53,54],vec![1,2,9,10,17,18,25,26,33,34,41,42,49,50,51,52,53,54],vec![1,2,6,7,9,10,11,13,14,15,17,18,19,20,21,22,23,25,26,28,30,31,33,34,38,39,41,42,46,47,49,50,54,55],vec![1,2,5,6,9,10,11,13,14,17,18,19,20,21,22,25,26,28,29,30,33,34,37,38,41,42,45,46,49,50,53,54],vec![2,3,4,5,9,10,13,14,17,18,21,22,25,26,29,30,33,34,37,38,41,42,45,46,50,51,52,53],vec![1,2,3,4,5,9,10,13,14,17,18,21,22,25,26,27,28,29,33,34,41,42,49,50],vec![2,3,4,5,9,10,13,14,17,18,21,22,25,26,29,30,33,34,37,38,42,43,44,45,52,53,54],vec![1,2,3,4,5,9,10,13,14,17,18,21,22,25,26,27,28,29,33,34,37,38,41,42,45,46,49,50,53,54],vec![2,3,4,5,9,10,13,14,17,18,26,27,28,29,37,38,41,42,45,46,50,51,52,53],vec![1,2,3,4,5,6,11,12,19,20,27,28,35,36,43,44,51,52],vec![1,2,5,6,9,10,13,14,17,18,21,22,25,26,29,30,33,34,37,38,41,42,45,46,50,51,52,53],vec![1,2,5,6,9,10,13,14,17,18,21,22,25,26,29,30,33,34,37,38,42,43,44,45,51,52],vec![1,2,6,7,9,10,14,15,17,18,22,23,25,26,28,30,31,33,34,35,36,37,38,39,41,42,43,45,46,47,49,50,54,55],vec![1,2,5,6,9,10,13,14,18,19,20,21,27,28,34,35,36,37,41,42,45,46,49,50,53,54],vec![1,2,5,6,9,10,13,14,17,18,21,22,26,27,28,29,35,36,43,44,51,52],vec![1,2,3,4,5,6,13,14,20,21,27,28,34,35,41,42,49,50,51,52,53,54],vec![2,3,4,5,10,11,18,19,26,27,34,35,42,43,50,51,52,53],vec![0,1,9,10,18,19,27,28,36,37,45,46,54,55],vec![2,3,4,5,12,13,20,21,28,29,36,37,44,45,50,51,52,53],vec![3,4,10,11,12,13,17,18,21,22],vec![56,57,58,59,60,61,62,63],vec![2,3,11,12,20,21],vec![18,19,20,21,29,30,34,35,36,37,38,41,42,45,46,50,51,52,53,54],vec![1,2,9,10,17,18,19,20,21,25,26,29,30,33,34,37,38,41,42,45,46,49,50,51,52,53],vec![18,19,20,21,25,26,29,30,33,34,41,42,45,46,50,51,52,53],vec![5,6,13,14,18,19,20,21,22,25,26,29,30,33,34,37,38,41,42,45,46,50,51,52,53,54],vec![18,19,20,21,25,26,29,30,33,34,35,36,37,38,41,42,50,51,52,53,54],vec![3,4,5,10,11,18,19,25,26,27,28,29,34,35,42,43,50,51],vec![18,19,20,21,22,25,26,29,30,33,34,37,38,42,43,44,45,46,53,54,57,58,59,60,61],vec![1,2,9,10,17,18,19,20,21,25,26,29,30,33,34,37,38,41,42,45,46,49,50,53,54],vec![3,4,18,19,20,27,28,35,36,43,44,50,51,52,53],vec![4,5,19,20,21,28,29,36,37,44,45,52,53,58,59,60],vec![1,2,9,10,17,18,21,22,25,26,29,30,33,34,35,36,37,41,42,45,46,49,50,53,54],vec![2,3,4,11,12,19,20,27,28,35,36,43,44,50,51,52,53],vec![17,18,19,21,22,25,26,27,28,29,30,31,33,34,36,38,39,41,42,44,46,47,49,50,54,55],vec![17,18,19,20,21,25,26,29,30,33,34,37,38,41,42,45,46,49,50,53,54],vec![18,19,20,21,25,26,29,30,33,34,37,38,41,42,45,46,50,51,52,53],vec![17,18,19,20,21,25,26,29,30,33,34,37,38,41,42,43,44,45,49,50,57,58],vec![18,19,20,21,22,25,26,29,30,33,34,37,38,42,43,44,45,46,53,54,61,62],vec![17,18,19,20,21,25,26,29,30,33,34,41,42,49,50],vec![18,19,20,21,22,25,26,34,35,36,37,45,46,49,50,51,52,53],vec![11,12,17,18,19,20,21,22,27,28,35,36,43,44,52,53,54],vec![17,18,21,22,25,26,29,30,33,34,37,38,41,42,45,46,50,51,52,53,54],vec![17,18,21,22,25,26,29,30,33,34,37,38,42,43,44,45,51,52],vec![17,18,22,23,25,26,28,30,31,33,34,36,38,39,42,43,44,45,46,50,51,53,54],vec![17,18,21,22,26,27,28,29,35,36,42,43,44,45,49,50,53,54],vec![17,18,21,22,25,26,29,30,33,34,37,38,42,43,44,45,46,52,53,57,58,59,60],vec![17,18,19,20,21,22,28,29,35,36,42,43,49,50,51,52,53,54],vec![4,5,6,11,12,19,20,25,26,27,35,36,43,44,52,53,54],vec![3,4,11,12,19,20,27,28,35,36,43,44,51,52],vec![1,2,3,11,12,19,20,28,29,30,35,36,43,44,49,50,51],vec![18,19,22,23,25,26,28,30,31,33,34,37,38],vec![],vec![4,11,12,18,19,20,25,26,27,28,34,35,36,43,44,52],vec![3,11,12,19,20,21,27,28,29,30,35,36,37,43,44,51],vec![4,11,12,13,18,19,20,21,22,25,26,27,28,29,30,31],vec![25,26,27,28,29,30,31,34,35,36,37,38,43,44,45,52],vec![27,28,35,36],vec![19,20,26,27,28,29,34,35,36,37,43,44],vec![10,11,12,13,17,18,19,20,21,22,25,26,27,28,29,30,33,34,35,36,37,38,41,42,43,44,45,46,50,51,52,53],vec![2,3,4,5,9,10,11,12,13,14,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,49,50,51,52,53,54,58,59,60,61],vec![27,28,29,30,31,35,36,37,38,39,43,44,51,52,59,60],vec![24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],vec![24,25,26,27,28,32,33,34,35,36,43,44,51,52,59,60],vec![3,4,11,12,19,20,27,28,35,36,43,44,51,52,59,60],vec![3,4,11,12,19,20,27,28,29,30,31,35,36,37,38,39],vec![3,4,11,12,19,20,24,25,26,27,28,32,33,34,35,36],vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],vec![0,2,4,6,9,11,13,15,16,18,20,22,25,27,29,31,32,34,36,38,41,43,45,47,48,50,52,54,57,59,61,63],vec![10,11,12,13,14,17,18,22,23,25,26,30,31,33,34,38,39,42,43,45,46,49,50,51,53,54,55],vec![17,18,21,22,25,26,29,30,33,34,37,38,41,42,43,44,45,46,47,49,50,57,58],vec![48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],vec![32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63],vec![0,1,8,9,16,17,24,25,32,33,40,41,48,49,56,57],vec![0,1,2,3,8,9,10,11,16,17,18,19,24,25,26,27,32,33,34,35,40,41,42,43,48,49,50,51,56,57,58,59],vec![29,30,31,36,37,38,39,43,44,45,51,52,59,60],vec![24,25,26,32,33,34,35,42,43,44,51,52,59,60],vec![3,4,11,12,19,20,21,28,29,30,31,37,38,39],vec![3,4,11,12,18,19,20,24,25,26,27,32,33,34],vec![3,4,11,12,19,20,27,28,29,30,31,35,36,37,38,39,43,44,51,52,59,60],vec![3,4,11,12,19,20,24,25,26,27,28,32,33,34,35,36,43,44,51,52,59,60],vec![24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,43,44,51,52,59,60],vec![3,4,11,12,19,20,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39],vec![7,14,15,21,22,25,26,28,29,33,34,35,36,41,42,43,49,50],vec![2,3,4,5,6,9,10,12,14,15,17,18,20,22,23,25,26,27,28,29,30,31,33,35,36,37,39,41,42,46,47,50,51,52,53,54]];
        for x in 0..100 * 100 {
            //write_pixel(&mut stream, SCREEN_WIDTH - x/100, x%100, "FFFFFF".to_string());
        }
        // print_letter(&mut stream, &alphabet, char_to_index('H') + 33, 0*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('E') + 33, 1*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('L') + 33, 2*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('L') + 33, 3*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('O') + 33, 4*64, 0, 8);
        // print_letter(&mut stream, &alphabet, 12, 5*64, 0, 8);
        // print_letter(&mut stream, &alphabet, 0, 6*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('W') + 33, 7*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('O') + 33, 8*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('R') + 33, 9*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('L') + 33, 10*64, 0, 8);
        // print_letter(&mut stream, &alphabet, char_to_index('D') + 33, 11*64, 0, 8);
        // print_letter(&mut stream, &alphabet, 1, 12*64, 0, 8);
        stream.write(print_letter(&alphabet, 1 + 16, 0 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 9 + 16, 1 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 2 + 16, 2 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 14, 3 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 1 + 16, 4 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 6 + 16, 5 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 8 + 16, 6 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 14, 7 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 0 + 16, 8 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 14, 9 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 9 + 16, 10 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 6 + 16, 11 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 26, 12 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 1 + 16, 13 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 3 + 16, 14 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 3 + 16, 15 * 56, 0, 7));
        stream.write(print_letter(&alphabet, 7 + 16, 16 * 56, 0, 7));
        // for y in 0..12 {
        //     for x in 0..20 {
        //         let i = (y*10 + x) % (32*4);
        //         print_letter(&mut stream, &alphabet, i, (x*48) as i32, (y*48) as i32);
        //     }
        // }
    }
    println!("Terminated.");
}
