use std::fs;
use substring::Substring;


struct CubeItem<'a> {
    qty: i32,
    color: &'a str
}

trait ValidItem {
    fn is_item_valid(&self, cube_item: &CubeItem) -> bool;
}

impl ValidItem for CubeItem <'_> {
    fn is_item_valid(&self, cube_item: &CubeItem) -> bool {
        return self.qty <= cube_item.qty && self.color == cube_item.color;
    }
}

pub fn _solve() -> i32 {

    let contents = fs::read_to_string("input-cube.txt").expect("Should have been able to read input file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let valid_red = CubeItem { qty: 12, color: "red"};
    let valid_green = CubeItem { qty: 13, color: "green"};
    let valid_blue = CubeItem { qty: 14, color: "blue"};

    let mut game_id_sum = 0;

    for line in lines.iter() {
         let mut valid_game: bool = true;
        let id_marker = line.find(":").expect("Couldn't find ID Marker : ");
        let game_id = line.substring(4, id_marker);
        let game_contents = line.substring(id_marker+1, line.len());
        let sub_groups = game_contents.split(";").collect::<Vec<&str>>(); // (3 blue, 4 red)

        for group in sub_groups.iter() {
            let items = group.split(",").collect::<Vec<&str>>();
            for item in items.iter() {

                let item_details = item.trim().split(" ").collect::<Vec<&str>>();     
                let curr_item = CubeItem { qty: item_details[0].parse::<i32>().unwrap(), color: item_details[1]};
                if !curr_item.is_item_valid(&valid_blue) && !curr_item.is_item_valid(&valid_green) && !curr_item.is_item_valid(&valid_red) {                        
                    valid_game = false;
                    break
                }
            }
        }

        if valid_game {
            game_id_sum += game_id.trim().parse::<i32>().unwrap();
        }
    }


    
    game_id_sum
}


pub fn _solve_part_2() -> i32 {

    let contents = fs::read_to_string("input-cube.txt").expect("Should have been able to read input file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut game_id_sum = 0;
   
    for line in lines.iter() {
        let id_marker = line.find(":").expect("Couldn't find ID Marker : ");

        let game_contents = line.substring(id_marker+1, line.len());
        let sub_groups = game_contents.split(";").collect::<Vec<&str>>(); // (3 blue, 4 red)
        let mut min_green = 0;
        let mut min_red = 0;
        let mut min_blue = 0;

        for group in sub_groups.iter() {
            let items = group.split(",").collect::<Vec<&str>>();
            for item in items.iter() {
               
                let item_details = item.trim().split(" ").collect::<Vec<&str>>();     
                let curr_item = CubeItem { qty: item_details[0].parse::<i32>().unwrap(), color: item_details[1]};
                match curr_item.color {
                    "red" => {
                        if curr_item.qty > min_red {
                            min_red = curr_item.qty;
                        }
                    },
                    "blue" => {
                        if curr_item.qty > min_blue {
                            min_blue = curr_item.qty;
                        }
                    },
                    "green" => {
                        if curr_item.qty > min_green {
                            min_green = curr_item.qty;
                        }
                    }
                    _ => continue
                }
            }
        }

       
       game_id_sum += min_blue * min_green * min_red
    }


    game_id_sum
}