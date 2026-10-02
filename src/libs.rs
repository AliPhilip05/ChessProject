//NOTE: we will call the function menu from the main file
//this way, we keep the main file unbloated.
use std::thread;
use std::time::Duration;

//NOTE: Change this time down to 0 during developing mode
const TIME:u64 = 2;
pub fn menu() {
    welcome_message();
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



fn game_menu() {
    let title = r#"
                A L I   A N D   R I C O   P R E S E N T S

                         G U E S S   T H E   E L O 
"#;
    println!("{title}");
    let bar_first = String::from("░░░░░░░░");
    let bar_second = String::from("████████");

    for i in 0..=5 {
        thread::sleep(Duration::from_secs(TIME));

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


