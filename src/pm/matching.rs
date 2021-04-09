pub fn test() {
    direct();
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn direct() {
    let dire = Direction::South;

    let d_str = match dire {
        Direction::East => {
            println!("East");
            "E"
        }
        Direction::North | Direction::South => {
            println!("South or North");
            "SN"
        }
        _ => "W",
    };

    println!("{:?}", d_str);
}
