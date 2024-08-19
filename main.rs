use std::io::{self, Write};

enum Room {
    Start,
    Hallway,
    Library,
    Kitchen,
    SecretRoom,
    Exit,
}

fn main() {
    let mut current_room = Room::Start;
    let mut inventory = vec![];
    let mut health = 100;
    let mut secret_unlocked = false;

    println!("Welcome to the Rusty Manor Adventure!");
    println!("Type 'help' for commands.");

    loop {
        match current_room {
            Room::Start => {
                println!("\nYou're in the entrance hall of an old manor. Cobwebs hang from the ceiling, and a faint draft chills your bones.");
                println!("There's a hallway ahead and a strange painting on the wall.");
                println!("Options: [forward, examine painting, exit]");
            }
            Room::Hallway => {
                println!("\nYou're in a long hallway. The air is thick with dust. There's a library to the left, a kitchen to the right, and a locked door at the end.");
                println!("Options: [left, right, forward, back]");
            }
            Room::Library => {
                println!("\nYou're in a dusty library filled with old books. The scent of aged paper fills the air.");
                if !inventory.contains(&"key") {
                    println!("You see a shiny key on a table. There is also an old book that seems out of place.");
                }
                println!("Options: [take key, read book, back]");
            }
            Room::Kitchen => {
                println!("\nYou're in a rustic kitchen. Pots and pans hang from the ceiling, and a locked door is at the far end.");
                println!("There is a faint smell of gas here.");
                if !secret_unlocked {
                    println!("You also notice some loose tiles on the floor.");
                }
                println!("Options: [unlock door, examine tiles, back]");
            }
            Room::SecretRoom => {
                println!("\nYou've discovered a hidden room beneath the kitchen. It's dark and cold, with a treasure chest in the corner.");
                println!("Options: [open chest, back]");
            }
            Room::Exit => {
                println!("\nCongratulations! You've escaped the Rusty Manor.");
                return;
            }
        }

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();

        match command.as_str() {
            "help" => {
                println!("Commands: forward, back, left, right, take key, read book, examine painting, unlock door, examine tiles, open chest, inventory, health, exit");
            }
            "inventory" => {
                if inventory.is_empty() {
                    println!("Your inventory is empty.");
                } else {
                    println!("Inventory: {:?}", inventory);
                }
            }
            "health" => {
                println!("Your health: {}%", health);
            }
            "forward" => {
                match current_room {
                    Room::Start => current_room = Room::Hallway,
                    Room::Hallway => {
                        if secret_unlocked {
                            current_room = Room::SecretRoom;
                        } else {
                            println!("The door is locked. Maybe there's a way to unlock it.");
                        }
                    }
                    _ => println!("You can't go forward here."),
                }
            }
            "back" => {
                match current_room {
                    Room::Hallway => current_room = Room::Start,
                    Room::Library | Room::Kitchen | Room::SecretRoom => current_room = Room::Hallway,
                    _ => println!("You can't go back here."),
                }
            }
            "left" => {
                if let Room::Hallway = current_room {
                    current_room = Room::Library;
                } else {
                    println!("You can't go left here.");
                }
            }
            "right" => {
                if let Room::Hallway = current_room {
                    current_room = Room::Kitchen;
                } else {
                    println!("You can't go right here.");
                }
            }
            "take key" => {
                if let Room::Library = current_room {
                    if !inventory.contains(&"key") {
                        inventory.push("key");
                        println!("You picked up the key.");
                    } else {
                        println!("You already have the key.");
                    }
                } else {
                    println!("There's no key to take here.");
                }
            }
            "read book" => {
                if let Room::Library = current_room {
                    println!("You open the old book and find a passage about a secret room hidden beneath the kitchen.");
                    println!("Maybe there's more to the kitchen than meets the eye.");
                } else {
                    println!("There's nothing to read here.");
                }
            }
            "examine painting" => {
                if let Room::Start = current_room {
                    println!("The painting depicts a beautiful landscape. Upon closer inspection, you notice a small inscription: 'Seek and you shall find.'");
                } else {
                    println!("There's nothing special about this.");
                }
            }
            "unlock door" => {
                if let Room::Kitchen = current_room {
                    if inventory.contains(&"key") {
                        println!("You unlock the door with the key and step outside.");
                        current_room = Room::Exit;
                    } else {
                        println!("The door is locked, and you don't have the key.");
                    }
                } else {
                    println!("There's no door to unlock here.");
                }
            }
            "examine tiles" => {
                if let Room::Kitchen = current_room {
                    println!("You carefully move the tiles aside and discover a hidden trapdoor leading downwards.");
                    secret_unlocked = true;
                } else {
                    println!("There's nothing special here.");
                }
            }
            "open chest" => {
                if let Room::SecretRoom = current_room {
                    println!("You open the chest and find a pile of gold coins! You are rich beyond your wildest dreams.");
                    println!("Congratulations! You've found the true treasure of Rusty Manor and escaped!");
                    return;
                } else {
                    println!("There's nothing to open here.");
                }
            }
            "exit" => {
                println!("Thanks for playing!");
                return;
            }
            _ => println!("Invalid command. Type 'help' for a list of commands."),
        }
    }
}
