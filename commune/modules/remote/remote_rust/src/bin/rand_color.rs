use rand::Rng;
use std::time::Instant;

fn main(){
    let start_time = Instant::now();

    let rand_color:String = c_random_color();
    println!("rand_color is {}", rand_color);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);

    // Print the elapsed time in seconds and milliseconds
    println!("Elapsed time: {}.{:03} seconds for Rand_Color", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}

pub fn c_random_color() -> String {
    let colors = c_colors();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..colors.len());
    colors[index].to_string()
}

pub fn c_colors() -> Vec<&'static str> {
    vec![
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white", 
        "bright_black", "bright_red", "bright_green", "bright_yellow", "bright_blue", 
        "bright_magenta", "bright_cyan", "bright_white"
    ]
}
