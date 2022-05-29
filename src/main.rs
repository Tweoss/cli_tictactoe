use std::io::Write;

#[derive(Debug, Clone)]
struct BoardState {
    board: [[CellState; 3]; 3],
    player_turn: bool,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            board: [[CellState::Empty; 3]; 3],
            player_turn: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum CellState {
    Empty,
    Player,
    Bot,
}

#[derive(Debug, PartialEq)]
enum BoardResult {
    PlayerWin,
    BotWin,
    Draw,
    Playing,
}

impl BoardState {
    fn check_result(&self) -> BoardResult {
        let player_won = |c1, c2, c3| -> Option<bool> {
            match (c1, c2, c3) {
                (CellState::Player, CellState::Player, CellState::Player) => Some(true),
                (CellState::Bot, CellState::Bot, CellState::Bot) => Some(false),
                _ => None,
            }
        };
        let board = &self.board;
        player_won(board[0][0], board[0][1], board[0][2])
            .or_else(|| player_won(board[1][0], board[1][1], board[1][2]))
            .or_else(|| player_won(board[2][0], board[2][1], board[2][2]))
            .or_else(|| player_won(board[0][0], board[1][0], board[2][0]))
            .or_else(|| player_won(board[0][1], board[1][1], board[2][1]))
            .or_else(|| player_won(board[0][2], board[1][2], board[2][2]))
            .or_else(|| player_won(board[0][0], board[1][1], board[2][2]))
            .or_else(|| player_won(board[0][2], board[1][1], board[2][0]))
            .map(|a| {
                if a {
                    BoardResult::PlayerWin
                } else {
                    BoardResult::BotWin
                }
            })
            .unwrap_or_else(|| {
                // if all not empty, then must be all full => draw
                if board
                    .iter()
                    .all(|row| row.iter().all(|cell| cell != &CellState::Empty))
                {
                    BoardResult::Draw
                } else {
                    BoardResult::Playing
                }
            })
    }

    // -1 if player wins, 1 if bot wins, 0 if draw
    fn min_max(&self) -> i32 {
        match self.check_result() {
            BoardResult::PlayerWin => return -1,
            BoardResult::BotWin => return 1,
            BoardResult::Draw => return 0,
            BoardResult::Playing => {}
        };
        let mut best = None;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == CellState::Empty {
                    let mut new_board = self.clone();
                    new_board.board[i][j] = if self.player_turn {
                        CellState::Player
                    } else {
                        CellState::Bot
                    };
                    new_board.player_turn = !new_board.player_turn;
                    let result = new_board.min_max();
                    if self.player_turn {
                        if best.is_none() || best.unwrap() > result {
                            best = Some(result);
                        }
                    } else {
                        if best.is_none() || best.unwrap() < result {
                            best = Some(result);
                        }
                    }
                }
            }
        }
        best.unwrap()
    }
    fn print(&self) {
        for row in &self.board {
            for cell in row {
                match cell {
                    CellState::Empty => print!("-"),
                    CellState::Player => print!("X"),
                    CellState::Bot => print!("O"),
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut state = BoardState::default();

    while state.check_result() == BoardResult::Playing {
        if state.player_turn {
            print!("Enter row and column (0-2): ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut split = line.trim().split_whitespace();
            let col = split.next().unwrap().parse::<usize>().unwrap();
            let row = split.next().unwrap().parse::<usize>().unwrap();
            if state.board[row][col] == CellState::Empty {
                state.board[row][col] = CellState::Player;
                state.player_turn = false;
            } else {
                println!("Invalid move");
            }
        } else {
            let mut best_result = -2;
            let mut best_move = (0, 0);
            for i in 0..3 {
                for j in 0..3 {
                    if state.board[i][j] == CellState::Empty {
                        let mut state = state.clone();
                        state.board[i][j] = CellState::Bot;
                        state.player_turn = true;
                        let result = state.min_max();
                        if result > best_result {
                            best_result = result;
                            best_move = (i, j);
                        }
                    }
                }
            }
            println!("Bot move: {} {}. Expected result {}", best_move.0, best_move.1, best_result);
            state.board[best_move.0][best_move.1] = CellState::Bot;
            state.player_turn = true;
        }
        state.print();
    }
    println!("{:?}", state.check_result());
}
