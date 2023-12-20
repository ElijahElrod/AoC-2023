use std::fs;
use substring::Substring;

pub fn solve() -> i32 {

    let contents = fs::read_to_string("input-scratchcards.txt").expect("Should have been able to read input file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut point_total = 0;

    for line in lines.iter() {
         
        let id_marker = line.find(":").expect("Couldn't find ID Marker : ");
        let card_marker = line.find("|").expect("Couldn't find card separator");
        let winning_numbers = line.substring(id_marker, card_marker).trim().split(" ").collect::<Vec<&str>>();
        let card_numbers = line.substring(card_marker+1, line.len()).trim().split(" ").collect::<Vec<&str>>();
        let mut card_val = 0;
        for number in winning_numbers.iter() {
         
            if !number.is_empty() && card_numbers.contains(number) {
                if card_val == 0 {
                    card_val = 1
                } else {
                    card_val *= 2
                }
            }
        }
       
        point_total += card_val

    }

    point_total
}