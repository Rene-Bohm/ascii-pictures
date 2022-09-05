use image::{GenericImageView, Pixel}; 
//use std::{thread, time};



fn main() {
    
    
    //let ten_millis = time::Duration::from_millis(100);

    let img = image::open("src/pic/tonta.jpg").expect("File not found!");

    let mut display: Vec<Vec<u8>> = Vec::new();

    let mut row: Vec<u8> = Vec::new(); 
    
    let mut current_y = 0 as u32;

    for (_,y,pixel) in img.pixels(){
        
        let (r, g, b, _) = pixel.channels4();

        let l = (0.33 * (r as f64) + 0.587 * (g as f64) + 0.114 * (b as f64)) as u8;

        //thread::sleep(ten_millis);

        if current_y == y{

            row.push(l);

        }else{

            row.reverse();

            display.push(row);

            row = Vec::new();

            row.push(l);

            current_y = current_y + 1;

        }

    }

    render(display);

}

fn render(escape_vals: Vec<Vec<u8>>) {

    for row in escape_vals {

        let mut line = String::with_capacity(row.len());
        
        for column in row {
        
            let val = match column {

                0..=25 => ' ',
                26..=50 => '.',
                51..=75 => '•',
                76..=100 => '*',
                101..=125 => '+',
                126..=150 => 'x',
                151..= 175 => '$',
                176..=200 => '#',
                201..=225 => '~',
                226..=240 => 'o',
                _ => '%',
        
            };
        
            line.push(val);

        }

        println!("{}", line);
    }
}