////////////////////
/// Game display ///
////////////////////

fn get_player_representation(player: u8, characters: &(char, char)) -> char {
    match player {
        0 => ' ',
        1 => characters.0,
        2 => characters.1,
        _ => panic!(
            "Got invalid character number {}: should be 0, 1, or 2",
            player
        ),
    }
}

#[test]
fn test_player_representation() {
    let pls = ('O', 'X');
    assert_eq!('O', get_player_representation(1, &pls));
    assert_eq!('X', get_player_representation(2, &pls));
    assert_eq!(' ', get_player_representation(0, &pls));
}

#[test]
#[should_panic]
fn test_player_representation_panic() {
    let pls = ('O', 'X');
    get_player_representation(3, &pls);
}

fn print_board(board: &Board, players: &(char, char)) {
    assert_eq!(board.len(), 9);

    for i in 0..3 {
        if i != 0 {
            println!("---+---+---");
        }

        println!("   |   |   ");

        for j in 0..3 {
            if j != 0 {
                print!("|");
            }
            print!(" {} ", get_player_representation(board[i * 3 + j], players));
        }

        println!();
        println!("   |   |   ");
    }
}

//////////////////
//  Game Logic  //
//////////////////
type PlayerID = u8;
struct Board(Vec<PlayerID>);

enum GameStatus {
	Won(PlayerID),
	Draw,
	InProgress,
	Cancelled
}

impl Board {
    fn new() -> Board {
        Board(vec![0; 9])
    }

    fn len(&self) -> usize {
        (self.0).len()
    }

    fn play(&mut self, player: PlayerID, pos: usize) -> Result<(), String> {
        assert!(pos < 9);
        assert!(player == 1 || player == 2);

        if self[pos] == 0 {
            self[pos] = player;
            Ok(())
        } else {
            Err(format!("Position already occupied by player {}", self[pos]))
        }
    }

	fn none_empty(&self) -> bool {
		!(self.0).iter().any(|v| *v == 0)
	}
}

use std::ops::{Index, IndexMut};

impl Index<usize> for Board {
    type Output = PlayerID;

    fn index<'a>(&'a self, p: usize) -> &'a Self::Output {
        &(self.0)[p]
    }
}

impl Index<(usize, usize)> for Board {
    type Output = PlayerID;
    fn index<'a>(&'a self, p: (usize, usize)) -> &'a Self::Output {
        &self[p.0 * 3 + p.1]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut<'a>(&'a mut self, p: usize) -> &'a mut PlayerID {
        &mut (self.0)[p]
    }
}

#[test]
fn test_can_place_on_empty_spot() {
    let mut b = Board::new();

    for i in 0..9 {
        match b.play(i % 2 + 1, i as usize) {
            Err(string) => assert!(false, string),
            _ => (),
        }
    }

    match b {
        Board(content) => assert_eq!(content, vec![1, 2, 1, 2, 1, 2, 1, 2, 1]),
    }
}

#[test]
fn test_cannot_place_on_taken_spot() {
    let mut b = Board(vec![1, 1, 1, 2, 1, 1, 2, 1, 1]);

    for i in 0..9 {
        match b.play(1, i) {
            Ok(()) => assert!(false, "Should have failed"),
            _ => (),
        }
    }
}

#[test]
fn test_none_empty() {
	assert!(!Board(vec![0, 1, 1, 1, 0, 1, 0, 0, 0]).none_empty());
	assert!(Board(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]).none_empty());
}

fn process_player_turn(
    id: PlayerID,
    board: &mut Board,
    input_function: &(Fn(&Board, PlayerID) -> usize),
) {
    while let Err(str) = board.play(id, input_function(board, id)) {
        println!("Error: {}", str);
    }
}

use std::str::FromStr;

fn get_player_input(_: &Board, id: PlayerID) -> usize {

	let between = |x, a, b| { x >= a && x <= b };
    loop {
        let mut input = String::new();

		println!("Please choose a move for player {} (single digit from 1 to 9 or two comma-separated digits from 1 to 3):", id);
        std::io::stdin().read_line(&mut input).unwrap();

        let substrings: Vec<&str> = input.split(',').collect();

		match substrings.len() {
			1 => {
				let pos = usize::from_str(&substrings[0].trim());

				match pos {
					Ok(val) if between(val, 1, 9) => return val - 1,
					_ => continue,
				}
			},
			2 => {
				let x = usize::from_str(&substrings[0].trim());
				let y = usize::from_str(&substrings[1].trim());

				match (x, y) {
					(Ok(a), Ok(b)) if between(a, 1, 3) && between(b, 1, 3) => return (a - 1) * 3 + b - 1,
					_ => continue,
				}
			}
			_ => continue,
		}

    }
}

fn has_won(board: &Board, id: PlayerID) -> bool {

	let cell_value = |(row, col)| { if board[(col, row)] == id { 1 } else { 0 } };

	let row = |c1, c2| { (c2, c1) };
	let column = |c1, c2| { (c1, c2) };
	let diag1 = |_, c2| { (c2, c2) };
	let diag2 = |_, c2| { (2 - c2, c2) };

	let full = |locater: Box<Fn(usize, usize) -> (usize, usize)>| {
		Box::new(move |coord1| {
			(0..3).into_iter().fold(0, |sum, coord2| { sum + cell_value((*locater)(coord1, coord2)) }) == 3
		})
	};

	let any = |f: &Fn(usize) -> bool| {
		(0..3).into_iter().any(|i| {
			f(i)
		})};
		
	any(&*full(Box::new(row))) || any(&*full(Box::new(column))) || (*full(Box::new(diag1)))(0) || (*full(Box::new(diag2)))(0)
}

fn check_win_condition(board: &Board, id: PlayerID) -> GameStatus {
	if  has_won(board, id) {
		GameStatus::Won(id)
	}
	else if board.none_empty() {
		GameStatus::Draw
	}
	else {
		GameStatus::InProgress
	}
}

#[test]
fn test_win_condition() {
	for i in 1..3 {
		let bi = Board(vec![i, i, i, 0, 0, 0, 0, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board(vec![0, 0, 0, i, i, i, 0, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board(vec![0, 0, 0, 0, 0, 0, i, i, i]);
		assert!(has_won(&bi, i));

		let bi = Board(vec![i, 0, 0, i, 0, 0, i, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board(vec![0, i, 0, 0, i, 0, 0, i, 0]);
		assert!(has_won(&bi, i));
		let bi = Board(vec![0, 0, i, 0, 0, i, 0, 0, i]);
		assert!(has_won(&bi, i));

		let bi = Board(vec![0, 0, i, 0, i, 0, i, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board(vec![i, 0, 0, 0, i, 0, 0, 0, i]);
		assert!(has_won(&bi, i));
	}

	for i in 1..3 {
		let bi = Board(vec![0,i, 0, 0, i, 0, 0, 0, i]);
		assert!(!has_won(&bi, i));
		let bi = Board(vec![0, i, 0, 0, i, 0, i, 0, 0]);
		assert!(!has_won(&bi, i));
		let bi = Board(vec![0, i, 0, 0, i, 0, i, 0, i]);
		assert!(!has_won(&bi, i));
	}
}

fn run_game_loop(board: &mut Board, players: &(char, char)) -> GameStatus {
	let mut current_player: PlayerID = 2;

	loop {
		match check_win_condition(board, current_player) {
			GameStatus::InProgress => {
				current_player = if current_player == 1 { 2 } else { 1 };
				process_player_turn(current_player, board, &get_player_input);
				print_board(board, players);
			},
			val @ _ => return val,
		}
	}
}

fn main() {
    let players = ('O', 'X');

	
	let mut b = Board::new();

	print_board(&b, &players);
	let result = run_game_loop(&mut b, &players);

	match result {
		GameStatus::Won(winner) => println!("Player {} won!", winner),
		GameStatus::Draw => println!("It's a draw!"),
		_ => (),
	}
}
