use rand::Rng;
use std::collections::HashMap;

struct Room
{
    name: String,
    description: String,
    exits: HashMap<String, String>, // all the possible exits
    items: Vec<String>, // items in the room
}

struct Player
{
    current_room: String,
    inventory: Vec<String>,
    health_points: i32,
    magic_points: i32, 
    level: i32,
    experience_points: i32,
}

fn create_rooms() -> Vec<Room> 
{

    let mut entrance_exits = HashMap::new();
    entrance_exits.insert("north".to_string(), "Hallway".to_string());

    let mut hallway_exits = HashMap::new();
    hallway_exits.insert("south".to_string(), "Entrance".to_string());
    hallway_exits.inesrt("east".to_string(), "Armory".to_string());
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
        health_points: 100,
        magic_points: 50,
        level: 1,
        experience_points: 0,
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
        println!("Health: {} | Magic: {} | level: {} | XP: {}", player.health_points, player.magic_points, player.level, player.experience_points);

        let mut command = String::new();
        println!("Enter a command: ");
        std::io::stdin().read_line(&mut command).unwrap();
        let command = command.trim().to_lowercase();

        // traps appearance in a room
        let mut rng = rand::thread_rng();
        let trap_chance = rng.gen_range(0..100);
        if trap_chance < rng.gen_range(10..=30)
        {
            trigger_trap(player);
        }

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
            "cast spell" =>
            {
                cast_spell(player);
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

fn cast_spell(player: &mut Player)
{
    println!("You have casted a spell.");
    player.magic_points -= 5;
    if player.magic_points <= 0
    {
        println!("You're out of magic!");
    }
    else 
    {
        println!("You used 5 magic points. Current magic: {}", player.magic_points);    
        gain_experience(player, 20);
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

fn trigger_trap(player: &mut Player)
{
    println!("You've fallen into a trap!");
    player.health_points -= 10;
    if player.health_points <= 0
    {
        println!("You have died.");
    }
    else 
    {
        println!("You have lost 10 health points. Current health: {}", player.health_points);
        gain_experience(player, 10);
    }
}

fn gain_experience(player: &mut Player, xp:i32)
{
    player.experience_points += xp;
    println!("You have gained {} XP!", xp);

    let xp_for_next_level = player.level * 100;

    if player.experience_points >= xp_for_next_level 
    {
        player.level += 1;
        player.experience_points = 0;
        player.health_points += 20;
        player.magic_points += 10;
        println!("Congratulation! You've reached level {}!", player.level);
        println!("HP increased to {}, MP increased to {}", player.health_points, player.magic_points);
    }
}
fn main()
{
    let mut rooms = create_rooms();
    let mut player = initialize_player();
    game_loop(&mut rooms, &mut player);
}
