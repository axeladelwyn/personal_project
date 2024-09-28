struct Room
{
    name: String,
    description: String,
    exits: Vec<String>, // all the possible exits
    items: Vec<String>, // items in the room
}

struct Player
{
    current_room: String,
    inventory: Vec<String>,
}

fn create_rooms() -> Vec<Room> 
{
    vec!
    [
        Room
        {
            name: String::from("Entrance"),
            description: String::from("You are at the entrance of the dungeon"),
            exits: vec![String::from("north")],
            items: vec![String::from("torch")],
        },
        Room
        {
            name: String::from("Hallway"),
            description: String::from("A long, dark hallway."),
            exits: vec![String::from("south"), String::from("east")],
            items: vec![],
        },
        Room
        {
            name: String::from("Armory"),
            description: String::from("Old, used armory."),
            exits: vec![String::from("west"), String::from("north")],
            items: vec![String::from("sword"), String::from("shield")],
        }
    ]
}

fn initialize_player() -> Player
{
    Player
    {
        current_room: String::from("Entrance"),
        inventory: vec![],
    }
}

fn game_loop(rooms: &mut Vec<Room>, player: &mut Player)
{
    loop 
    {
        let current_room = rooms.iter_mut().find(|room| room.name == player.current_room).unwrap();
        println!("You are in: {}", current_room.name);
        println!("{}", current_room.description);
        println!("Exits: {:?}", current_room.exits);
        println!("Items in the room: {:?}", current_room.items);
        println!("Your inventory {:?}", player.inventory);

        let mut command = String::new();
        println!("Enter a command: ");
        std::io::stdin().read_line(&mut command).unwrap();
        let command = command.trim().to_lowercase();

        match command.as_str()
        {
            "exit" => break,
            cmd if cmd.starts_with("take ") =>
            {
                let item= cmd[5..].trim();
                take_item(item, current_room, player);
            }
            cmd if cmd.starts_with("drop ") =>
            {
                let item = cmd[5..].trim();
                drop_item(item, current_room, player);
            }
            "inventory" =>
            {
                println!("Your inventory: {:?}", player.inventory);
            }
            _ => println!("Unknown command!"),
        }
    }
}

fn take_item(item: &str, room: &mut Room, player: &mut Player)
{
    if room.items.contains(&item.to_string())
    {
        room.items.retain(|i| i != item);
        player.inventory.push(item.to_string());
        println!("You have taken the {}!", item);
    }
    else 
    {
        println!("The {} is not here.", item);    
    }
}

fn drop_item(item: &str, room: &mut Room, player: &mut Player)
{
    if player.inventory.contains(&item.to_string())
    {
        player.inventory.retain(|i| i != item);
        room.items.push(item.to_string());
        println!("You have dropped the {}!", item);

    }
    else 
    {
        println!("You don't have a {}.", item);
    }
}
fn main()
{
    let mut rooms = create_rooms();
    let mut player = initialize_player();
    game_loop(&mut rooms, &mut player);
}
