// {{{ Main
fn main() {
    //let a = vec!['0', '1', '2', '3', '4', '5'];

    let dice = vec![
        vec!['p', 'g', 'p', 'y', 'g', 'r',],
        vec!['r', 'p', 'r', 'r', 'g', 'y',],
        vec!['p', 'p', 'r', 'y', 'y', 'g',],
        vec!['r', 'r', 'y', 'g', 'p', 'y',],
    ];

    let a = &dice[0];
    nested_rotates(&dice);

    //print_dice(&a);
    //dbg!(sides_with_front(&a, 4, 0));
    //dbg!(sides_with_front(&a, 4, 1));
    //dbg!(sides_with_front(&a, 4, 2));
    //dbg!(sides_with_front(&a, 4, 3));
    //dbg!("{}", get_sides(vec!['0', '1', '2', '3', '4', '5'], 2));
    //dbg!("{}", get_sides(vec!['0', '1', '2', '3', '4', '5'], 0));
}
// }}}
// Print Methods {{{
fn print_dice(dice: &Vec<char>) {
    assert!(dice.len() == 6);
    let inside = "     ";
    let left_right = "+-----";
    let half = &inside[0..(inside.len()/2)];

    // Top part
    println!("{} {}+", inside, left_right);
    println!("{} |{}{}{}|", inside, half, dice[4], half);

    //Middle part
    println!("{}{}{}{}+", left_right, left_right, left_right, left_right);
    for x in 0..4 {
        print!("|{}{}{}", half, dice[x], half);
    }
    println!("|");
    println!("{}{}{}{}+", left_right, left_right, left_right, left_right);

    //Bottom part
    println!("{} |{}{}{}|", inside, half, dice[5], half);
    println!("{} {}+", inside, left_right);
}
// }}}
//{{{ Looking at dice
fn sides_with_front(dice: &Vec<char>, top: usize, front: usize) -> Vec<char> {
    //Takes top and front, then returns right to left numbers on the side of the dice
    let sides = get_sides(&dice, top);
    let index = sides.iter().position(|&x| x == dice[front]);
    match index {
        Some(ind) => (0..4).map(|x| sides[(ind + x) % 4]).collect(),
        None => panic!("Attempted to pick a front that is not in sides"),
    }
}
    

fn get_sides(dice: &Vec<char>, top: usize) -> Vec<char> {
    // Returns a vec of the sides with no given front
    // The .rev() stuff is possible because the sequence
    // will go backwards if it is upside down
    match top {
        0 => get_sides(dice, 2).iter().rev().map(|x| *x).collect(),
        1 => get_sides(dice, 3).iter().rev().map(|x| *x).collect(),
        2 => vec![dice[1], dice[5], dice[3], dice[4]],
        3 => vec![dice[0], dice[4], dice[2], dice[5]],
        4 => vec![dice[0], dice[1], dice[2], dice[3]],
        5 => get_sides(dice, 4).iter().rev().map(|x| *x).collect(),
        _ => panic!("Top called on an invalid number"),
    }
}


fn get_side_index(top: usize) -> Vec<usize> {
    // Returns a vec of the sides with no given front
    // The .rev() stuff is possible because the sequence
    // will go backwards if it is upside down
    match top {
        0 => get_side_index(2).iter().rev().map(|x| *x).collect(),
        1 => get_side_index(3).iter().rev().map(|x| *x).collect(),
        2 => vec![1, 5, 3, 4],
        3 => vec![0, 4, 2, 5],
        4 => vec![0, 1, 2, 3],
        5 => get_side_index(4).iter().rev().map(|x| *x).collect(),
        _ => panic!("Top called on an invalid number"),
    }
}

//}}}
// {{{ Extremely brute force method

fn nested_rotates(dice_to_match : &Vec<Vec<char>>) {
    //Take dice to look over, return list of all configs
    //such that no colors match

    //Start with 2 dice

    let mut i = 0;
    for top in 0..6 {
        println!("Top: {}", top);
        for front in get_side_index(top) {

            let bot_dice = sides_with_front(&dice_to_match[0], top, front);
            //println!("{}{}{}{}", bot_dice[0], bot_dice[1], bot_dice[2], bot_dice[3]);
            
            //Compare sides of current top,front to the next dice
            for top2 in 0..6 {
                for front2 in get_side_index(top2) {
                    //If a configuration works, print out both dice
                    let top_dice = sides_with_front(&dice_to_match[1], top2, front2);

                }
            }

        }
        println!();
    }
}

