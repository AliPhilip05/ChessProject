//NOTE: we will call the function menu from the main file
//this way, we keep the main file unbloated.
use std::thread;
use std::time::Duration;

//NOTE: Change this time down to 0 during developing mode
const TIME:u64 = 2;
struct Mode(String,String,String,String);

pub fn menu() {
    welcome_message();
}

//TODO: Turn the code into a vector intead and then return the vector to acess the element 
fn log(input:u64) -> String {
    match input {
        0 => return String::from("Guess The Elo   "),
        1 => return String::from("Guess The Rating"),
        2 => return String::from("Guess The Player"),
        3 => return String::from("Quit            "),
        _ => return String::from(""),
    };
}

fn menu_options() {
    let mut cursor:u32 = 0;
    let game = Mode("{log(0)}".to_string(),"log(1)".to_string(),"log(2)".to_string(),"log(3)".to_string());
    let start = String::from("║                     ");
    let end = String::from("                      ║");
    let menu1 = r#"
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║                         GAME MODE                            ║
║                                                              ║
"#;
    let menu2 = r#"

║                                                              ║
╚══════════════════════════════════════════════════════════════╝

                         Choose an option:
"#;
    let cursor_point: String = String::from(">  ");
    let cursor_non_point: String = String::from("   ");

    println!("{menu1}");

    for i in 0..4 {
        if i == cursor {
            println!("{start} {cursor_point} {} {end}", game.i);
        } else {
            println!("{start} {cursor_non_point} {} {end}", game.i);
        }
    }
    println!("{menu2}");

}

fn welcome_message() {
    //TODO: write a welcome message 
    //and welcome image.
   game_menu();
   thread::sleep(Duration::from_secs(TIME));

    let title = r#"
██     ██ ███████ ██       ██████  ██████  ███    ███ ███████ 
██     ██ ██      ██      ██      ██    ██ ████  ████ ██      
██  █  ██ █████   ██      ██      ██    ██ ██ ████ ██ █████   
██ ███ ██ ██      ██      ██      ██    ██ ██  ██  ██ ██      
 ███ ███  ███████ ███████  ██████  ██████  ██      ██ ███████ 

                    ████████  ██████                      
                       ██    ██    ██                     
                       ██    ██    ██                     
                       ██    ██    ██                     
                       ██     ██████                      

 ██████  ██    ██ ███████ ███████ ███████     ████████ ██   ██ ███████
██       ██    ██ ██      ██      ██             ██    ██   ██ ██     
██   ███ ██    ██ █████   ███████ ███████        ██    ███████ █████  
██    ██ ██    ██ ██           ██      ██        ██    ██   ██ ██     
 ██████   ██████  ███████ ███████ ███████        ██    ██   ██ ███████

                         ███████ ██       ██████
                         ██      ██      ██    ██
                         █████   ██      ██    ██
                         ██      ██      ██    ██
                         ███████ ███████  ██████
"#;

println!("{}", title);


}

//NOTE: This function will produce a loading bar 
fn loading_bar(time:u64) {
    let bar_first = String::from("░░░░░░░░");
    let bar_second = String::from("████████");

    for i in 0..= 5 {
        thread::sleep(Duration::from_secs(time));
        let mut bar:String = String::new();

        for _ in 0..i {
            bar += &bar_second;
        } 
        for _ in i..5 {
            bar += &bar_first;
        }
        let percentage = i * 20;
        println!(
        "
    L O A D I N G . . .

    {bar} {percentage}%
    "); 
    }
}

fn game_menu() {
    let title = r#"
                A L I   P H I L I P   A N D   E N R I Q U E   S A N C H E Z   P R E S E N T

                         G U E S S   T H E   E L O 
"#;
    println!("{title}");
    loading_bar(TIME);
    menu_options();
} 


