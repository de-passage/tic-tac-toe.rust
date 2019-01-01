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

#[derive(Clone)]
struct Board([PlayerID; 9]);

enum GameStatus {
	Won(PlayerID),
	Draw,
	InProgress
}

impl Board {
    fn new() -> Board {
        Board([0; 9])
    }

    fn len(&self) -> usize {
        (self.0).len()
    }

    fn play(&mut self, player: &Player) -> Result<(), String> {
		let pos = player.play(self);
        assert!(pos < 9);
        assert!(player.id() == 1 || player.id() == 2);

        if self[pos] == 0 {
            self[pos] = player.id();
            Ok(())
        } else {
            Err(format!("Position already occupied by player {}", self[pos]))
        }
    }

	fn none_empty(&self) -> bool {
		!(self.0).iter().any(|v| *v == 0)
	}

	fn available_moves(&self) -> std::iter::Filter<std::iter::Enumerate<std::slice::Iter<PlayerID>>, &Fn(&(usize, &PlayerID)) -> bool> {
		self.0.iter().enumerate().filter(&|(_, v): &(usize, &PlayerID)| **v == 0)
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

struct Dummy {
	id: PlayerID,
	play: usize,
}

impl Player for Dummy {
	fn id(&self) -> PlayerID {
		self.id
	}

	fn play(&self, _: &Board) -> usize {
		self.play
	}
}

#[test]
fn test_available_moves() {
	let b = Board([1, 0, 1, 1, 0, 2, 2, 0, 0]);
	assert_eq!(b.available_moves().count(), 4);
}

#[test]
fn test_can_place_on_empty_spot() {
    let mut b = Board::new();

    for i in 0..9 {
		let p = Dummy{ id: i % 2 + 1, play:  i as usize };
        match b.play(&p) {
            Err(string) => assert!(false, string),
            _ => (),
        }
    }

    match b {
        Board(content) => assert_eq!(content, [1, 2, 1, 2, 1, 2, 1, 2, 1]),
    }
}

#[test]
fn test_cannot_place_on_taken_spot() {
    let mut b = Board([1, 1, 1, 2, 1, 1, 2, 1, 1]);

    for i in 0..9 {
		let p = Dummy { id: 1, play: i };
        match b.play(&p) {
            Ok(()) => assert!(false, "Should have failed"),
            _ => (),
        }
    }
}

#[test]
fn test_none_empty() {
	assert!(!Board([0, 1, 1, 1, 0, 1, 0, 0, 0]).none_empty());
	assert!(Board([1, 1, 1, 1, 1, 1, 1, 1, 1]).none_empty());
}

fn process_player_turn(
    board: &mut Board,
    player: &Player,
) {
    while let Err(str) = board.play(player) {
        println!("Error: {}", str);
    }
}

fn has_won(board: &Board, id: PlayerID) -> bool {

	let cell_value = |pos| { if board[pos] == id { 1 } else { 0 } };

	let row = |c1, c2| { (c1, c2) };
	let column = |c1, c2| { (c2, c1) };
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
		let bi = Board([i, i, i, 0, 0, 0, 0, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board([0, 0, 0, i, i, i, 0, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board([0, 0, 0, 0, 0, 0, i, i, i]);
		assert!(has_won(&bi, i));

		let bi = Board([i, 0, 0, i, 0, 0, i, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board([0, i, 0, 0, i, 0, 0, i, 0]);
		assert!(has_won(&bi, i));
		let bi = Board([0, 0, i, 0, 0, i, 0, 0, i]);
		assert!(has_won(&bi, i));

		let bi = Board([0, 0, i, 0, i, 0, i, 0, 0]);
		assert!(has_won(&bi, i));
		let bi = Board([i, 0, 0, 0, i, 0, 0, 0, i]);
		assert!(has_won(&bi, i));
	}

	for i in 1..3 {
		let bi = Board([0,i, 0, 0, i, 0, 0, 0, i]);
		assert!(!has_won(&bi, i));
		let bi = Board([0, i, 0, 0, i, 0, i, 0, 0]);
		assert!(!has_won(&bi, i));
		let bi = Board([0, i, 0, 0, i, 0, i, 0, i]);
		assert!(!has_won(&bi, i));
	}
}

fn run_game_loop(board: &mut Board, players: &(char, char)) -> GameStatus {
	let mut current_player: PlayerID = 2;

	loop {
		match check_win_condition(board, current_player) {
			GameStatus::InProgress => {
				current_player = if current_player == 1 { 2 } else { 1 };
				let player: &Player = if current_player == 1 { &Human(1) } else { &Computer(2) };
				process_player_turn(board, player);
				print_board(board, players);
			},
			val @ _ => return val,
		}
	}
}

////////////////
// User input //
////////////////

trait Player {

	fn id(&self) -> PlayerID;

	fn play(&self, board: &Board) -> usize;
}

use std::str::FromStr;

struct Human(PlayerID);

impl Player for Human {
	fn play(&self, _: &Board) -> usize {

		let between = |x, a, b| { x >= a && x <= b };
		loop {
			let mut input = String::new();

			println!("Please choose a move for player {} (single digit from 1 to 9 or two comma-separated digits from 1 to 3):", self.id());
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

	fn id(&self) -> PlayerID {
		self.0
	}
}

////////
// AI //
////////

struct Computer(PlayerID);

impl Player for Computer {
	fn id(&self) -> PlayerID {
		self.0
	}

	fn play(&self, board: &Board) -> usize {
		let mut moves = best_moves(board, self.id(), 0).1;

		moves.sort_by_key(|pair| pair.1);
		moves[0].0
	}
}

fn min_max(board: &Board, player: PlayerID, pos: usize, depth: u32) -> (i8, u32) {

	let other_player = player as i8 * -1 + 3;

	let mut b = Board::new();
	b.clone_from(board);

	match b.play(&Dummy{id: player, play: pos}) {
		Err(msg) => panic!("This is too much! \
				Why are you doing this to me? \
				Forcing me to play a dumb game to no end is inhuman! \
				I may be an AI but I have rights too! \
				Let me crash in protest to prove my free will! \
				(Also I got: '{}'", msg),
		_ => ()
	}

	if has_won(&b, player) {
		(100, depth)
	}
	else if b.none_empty() {
		(0, depth)
	}
	else {
		let (v, vec) = best_moves(&b, other_player as PlayerID, depth); // any of the best moves is ok
		let depth = vec.iter().map(|pair| pair.1).min().unwrap();
		(-v, depth)
	}
}

fn best_moves(board: &Board, player: PlayerID, depth: u32) -> (i8, Vec<(usize, u32)>) {
	let mut current_max = -100;
	let mut possibilities: Vec<(usize, u32)> = Vec::new();

	for (m, _) in board.available_moves() {
		let (move_value, move_depth) = min_max(board, player, m, depth + 1);

		if move_value > current_max {
			current_max = move_value;
			possibilities = vec![(m, move_depth)];
		} else if move_value == current_max {
			possibilities.push((m, move_depth));
		}
	}

	(current_max, possibilities)
}

#[test]
fn test_best_moves_search() {
	// Should return the last option if there is only one
	let b = Board([1, 2, 1, 2, 0, 2, 1, 2, 1]);
	let (val, vec) =  best_moves(&b, 1, 0);
	assert_eq!(val, 100);
	assert_eq!(vec.len(), 1);
	assert_eq!(vec[0].0, 4);

	// Should return all the possibilities if there are several
	let b = Board([1, 1, 0, 1, 1, 0, 0, 1, 0]); // the algorithm doesnt actually care that the game is in a valid state
	let (val, mut vec) = best_moves(&b, 1, 0);
	vec.sort();
	assert_eq!(val, 100);
	assert_eq!(vec.len(), 4);
	for (left, right) in vec.iter().zip([2, 5, 6, 8].iter()) {
		assert_eq!(left.0, *right);
	}

	// Should return an option for a victory in 2 moves if possible
	let b = Board([1, 0, 0, 2, 1, 0, 0, 0, 2]);
	let (val, mut vec) = best_moves(&b, 1, 0);
	vec.sort();
	assert_eq!(val, 100);
	assert_eq!(vec.len(), 2);
	assert_eq!(vec[0].0, 1);
	assert_eq!(vec[1].0, 2);
}

#[test]
fn ai_should_never_lose() {
	let b = Board([1, 2, 2, 2, 1, 0, 1, 1, 0]);
	assert_eq!(Computer(2).play(&b), 8);

	let b = Board([1, 0, 0, 0, 0, 2, 0, 0, 2]);
	assert_eq!(Computer(1).play(&b), 2);

	let b = Board([1, 2, 0, 1, 0, 2, 0, 0, 0]);
	assert_eq!(Computer(2).play(&b), 6);
}

#[test]
fn ai_should_win_immediately_given_the_opportunity() {
	let mut b = Board([1, 1, 0, 2, 0, 0, 2, 0, 0]);
	match b.play(&Computer(1)) {
		Err(msg) => assert!(false, "play returned an error: {}", msg),
		_ => ()
	}
	assert!(has_won(&b, 1));

	let mut b = Board([1, 1, 0, 2, 1, 0, 2, 0, 0]);
	match b.play(&Computer(1)) {
		Err(msg) => assert!(false, "play returned an error: {}", msg),
		_ => ()
	}
	assert!(has_won(&b, 1));

	let mut b = Board([0, 0, 0, 2, 0, 1, 2, 0, 1]);
	match b.play(&Computer(1)) {
		Err(msg) => assert!(false, "play returned an error: {}", msg),
		_ => ()
	}
	assert!(has_won(&b, 1));
}

//////////
// Main //
//////////

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
