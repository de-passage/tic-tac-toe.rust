////////////////////
/// Game display ///
////////////////////

fn get_player_representation(player: u8, characters: (char, char)) -> char {
	match player {
		0 => ' ',
		1 => characters.0,
		2 => characters.1,
		_ => panic!("Got invalid character number {}: should be 0, 1, or 2", player)
	}
}

#[test]
fn test_player_representation() {
	let pls = ('O', 'X');
	assert_eq!('O', get_player_representation(1, pls));
	assert_eq!('X', get_player_representation(2, pls));
	assert_eq!(' ', get_player_representation(0, pls));
}

#[test]
#[should_panic]
fn test_player_representation_panic() {
	let pls = ('O', 'X');
	get_player_representation(3, pls);
}

fn print_board(board: &Board, players: (char, char)) {

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
		}
		else {
			Err(format!("Position already occupied by player {}", self[pos]))
		}
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
		Board(content) => assert_eq!(content, vec![1, 2, 1, 2, 1, 2, 1, 2, 1])
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

fn process_player_turn(id: PlayerID, board: &mut Board, input_function: &(Fn(&Board, PlayerID) -> usize) ) {
	while let Err(str) = board.play(id, input_function(board, id)) {
		println!("Error: {}", str);
	}
}

fn run_game_loop(board: &mut Board) {
	
}


fn main() {
}
