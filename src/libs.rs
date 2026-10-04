//NOTE: we will call the function menu from the main file
//this way, we keep the main file unbloated.
use std::thread;
use std::time::Duration;

//NOTE: Change this time down to 0 during developing mode
const TIME:u64 = 2;

//NOTE: This function might be pointless 
pub fn menu() {
    welcome_message();
}

//NOTE: This function calls the welcome message 
fn welcome_message() {
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

// NOTE: This is the function that is called from main that runs the game. 
fn game_menu() {
    let title = r#"
                A L I   P H I L I P   A N D   E N R I Q U E   S A N C H E Z   P R E S E N T

                         G U E S S   T H E   E L O 
"#;
    println!("{title}");
    loading_bar(TIME);
    menu_options();
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

//NOTE: This function produces the menu-options. WIP 
fn menu_options() {
    let mut cursor:usize = 0;
    let cursor_point: String = String::from(">  ");
    let cursor_non_point: String = String::from("   ");
    let mut vec: Vec<String> = Vec::new();
    for i in 0..3 {
        vec.push(log(i));
    }

    let start = String::from("║                     ");
    let end = String::from("                   ║");
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
    
    println!("{menu1}");
    let iterator = vec.len();
    for i in 0..iterator {
        if i == cursor {
            println!("{start} {cursor_point} {} {end}", vec[i].to_string());
        } else {
            println!("{start} {cursor_non_point} {} {end}", vec[i]);
        }
    }
    println!("{menu2}");

}

//NOTE: This function contains all of the menu options.
fn log(input:u64) -> String {
    match input {
        0 => return String::from("Guess The Elo   "),
        1 => return String::from("Guess The Rating"),
        2 => return String::from("Guess The Player"),
        3 => return String::from("Quit            "),
        _ => return String::from(""),
    };
}
