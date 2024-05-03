pub mod rand_color{
    use rand::Rng;

fn main(){
    let rand_color:String = c_random_color();
    println!("rand_color is {} and", rand_color);
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
}
