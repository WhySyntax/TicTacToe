extern crate functions;
use functions::functions::input;

fn turn() -> [u8;2] {
    let returned_value = [input::u8_input("which row do you want to put your symbol in"),input::u8_input("which column do you want to put your symbol in")];
    return returned_value;
}

fn check_if_win_or_draw(gameArray:[[&str;4];4]) -> bool {
    if !(gameArray[1].contains(&" ")||gameArray[2].contains(&" ")||gameArray[3].contains(&" ")) {
        println!("It's a tie");
        return true;
    }
    for i in 1..4 {
        if ((gameArray[i][1]==gameArray[i][2])&&(gameArray[i][2]==gameArray[i][3]))||((gameArray[1][i]==gameArray[2][i])&&(gameArray[2][i]==gameArray[3][i])) {
            if gameArray[i][i]!=" " {
                println!("{} wins", gameArray[i][i]);
                return true;
            }
        }
    }

    if (gameArray[1][1]==gameArray[2][2])&&(gameArray[2][2]==gameArray[3][3]) {
        if gameArray[2][2]!=" " {
            println!("{} wins", gameArray[1][1]);
            return true;
        }
    }
    return false;
}

fn main() {
    //setting up
    let mut game_array:[[&str;4];4] = [["x","1","2","3"],["1"," "," "," "],["2"," "," "," "],["3"," "," "," "]];
    println!("{:?}\n{:?}\n{:?}\n{:?}",game_array[0],game_array[1],game_array[2],game_array[3]);
    let mut win = false;

    //the game itself
    while !win {
        //x turns
        println!("X's turn");
        let mut this_turn = turn();
        if game_array[this_turn[0] as usize][this_turn[1] as usize]==" " {
            game_array[this_turn[0] as usize][this_turn[1] as usize]="x";
        } else {
            println!("already taken, try again");
            let mut this_turn = turn();
            if game_array[this_turn[0] as usize][this_turn[1] as usize]==" " {
                game_array[this_turn[0] as usize][this_turn[1] as usize]="x";
            } else {println!("already taken, your turn is now skipped")}
        }
        println!("Here's the board\n{:?}\n{:?}\n{:?}\n{:?}",game_array[0],game_array[1],game_array[2],game_array[3]);
        win = check_if_win_or_draw(game_array);
        if win {break}

        println!("O's turn");
        let mut this_turn = turn();
        if game_array[this_turn[0] as usize][this_turn[1] as usize]==" " {
            game_array[this_turn[0] as usize][this_turn[1] as usize]="O";
        } else {
            println!("already taken, try again");
            let mut this_turn = turn();
            if game_array[this_turn[0] as usize][this_turn[1] as usize]==" " {
                game_array[this_turn[0] as usize][this_turn[1] as usize]="x";
            } else {println!("already taken, your turn is now skipped")}
        }
        println!("Here's the board\n{:?}\n{:?}\n{:?}\n{:?}",game_array[0],game_array[1],game_array[2],game_array[3]);
        win = check_if_win_or_draw(game_array);
    }
}
