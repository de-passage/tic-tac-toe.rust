//////////////////
//  Game Logic  //
//////////////////
mod game_logic {

use std::ops::{Index, IndexMut};

pub type PlayerID = u8;

#[derive(Clone)]
pub struct Board([PlayerID; 9]);

pub enum GameStatus {
	Won(PlayerID),
	Draw,
	InProgress
}

trait Player {

	fn id(&self) -> PlayerID;

	fn play(&self, board: &Board) -> usize;
}

impl Board {
    pub fn new() -> Board {
        Board([0; 9])
    }

    pub fn len(&self) -> usize {
        (self.0).len()
    }

    pub fn play(&mut self, player: &Player) -> Result<(), String> {
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

	pub fn none_empty(&self) -> bool {
		!(self.0).iter().any(|v| *v == 0)
	}

	pub fn available_moves(&self) -> std::iter::Filter<std::iter::Enumerate<std::slice::Iter<PlayerID>>, &Fn(&(usize, &PlayerID)) -> bool> {
		self.0.iter().enumerate().filter(&|(_, v): &(usize, &PlayerID)| **v == 0)
	}
}

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

pub struct Dummy {
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

pub fn process_player_turn(
    board: &mut Board,
    player: &Player,
) {
    while let Err(str) = board.play(player) {
        println!("Error: {}", str);
    }
}

pub fn has_won(board: &Board, id: PlayerID) -> bool {

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

pub fn check_win_condition(board: &Board, id: PlayerID) -> GameStatus {
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

pub fn run_game_loop(board: &mut Board, players: &(char, char)) -> GameStatus {
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

}
