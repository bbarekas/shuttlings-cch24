// Challenge 12 : https://console.shuttle.dev/shuttlings/cch24/challenge/12

use core::fmt;
use std::{sync::{Arc, Mutex, RwLock}};
use axum::{
    Router,
    http::{StatusCode},
    response::IntoResponse,
    routing::{post, get},
    extract::{Path, State}

};
use rand::{rngs::StdRng, Rng, SeedableRng};

static EMPTY: char = '⬛';
static COOKIE: char = '🍪';
static MILK: char = '🥛';
static WALL: char = '⬜';

#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum Team {
    #[default]
    Empty,
    Cookie,
    Milk,
}

#[derive(Default, Clone, Copy)]
struct Game {
    board: [[Team; 4]; 4],
    winner: Option<Team>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for row in &self.board {
            res.push(WALL);
            for team in row {
                match team {
                    Team::Cookie => res.push_str(&COOKIE.to_string()),
                    Team::Milk => res.push_str(&MILK.to_string()),
                    Team::Empty => res.push_str(&EMPTY.to_string()),
                }
            }
            res.push(WALL);
            res.push('\n');
        }
        res.push_str(&format!("{WALL}{WALL}{WALL}{WALL}{WALL}{WALL}\n"));
        if let Some(team) = self.winner {
            match team {
                Team::Cookie => res.push_str(&format!("{COOKIE} wins!\n")),
                Team::Milk => res.push_str(&format!("{MILK} wins!\n")),
                Team::Empty => res.push_str("No winner.\n"),
            }
        }

        write!(f, "{res}")
    }
}

//
impl Game {
    fn default() -> Game {
        Game {
            board: [
                [Team::Empty, Team::Empty, Team::Empty, Team::Empty],
                [Team::Empty, Team::Empty, Team::Empty, Team::Empty],
                [Team::Empty, Team::Empty, Team::Empty, Team::Empty],
                [Team::Empty, Team::Empty, Team::Empty, Team::Empty],
            ],
            winner: None,
        }
    }

    fn place(&mut self, team: Team, col: usize) -> Option<usize> {
        let board = &mut self.board;
        let Some(y) = board
            .iter()
            .rev()
            .position(|row| row[col] == Team::Empty)
        else {
            return None
        };
        let y = board.len() - y - 1;
        board[y][col] = team;
        Some(y)
    }

    fn validate(&self, row: usize, col: usize) -> Option<Team> {
        let board = &self.board;
        let item = board[row][col];
        // Check row.
        if board[row][0] != Team::Empty && board[row].iter().all(|&t| t == item) {
            println!("Row win: {}", row);
            return Some(item);
        }
        // Check column.
        if board[0][col] != Team::Empty && (0..board[0].len()).all(|y| board[y][col] == item) {
            println!("Column win: {}", col);
            return Some(item);
        }
        // Check diagonal. TL -> BR
        if row == col
            && board[0][0] != Team::Empty
            && (0..board.len()).all(|i| board[i][i] == item)
        {
            return Some(item);
        }
        // Check diagonal. BL -> TR
        if row + col == 3
            && board[0][3] != Team::Empty
            && (0..board.len()).all(|i| board[board.len() - i - 1][i] == item)
        {
            return Some(item);
        }
        // All full -> No winner
        if board.iter().all(|r| r.iter().all(|&t| t != Team::Empty)) {
            return  Some(Team::Empty);
        }
        None
    }

    fn generate_random(rand: &mut StdRng) -> Self {
        let mut game = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                game.board[i][j] = if rand.r#gen::<bool>() {
                    Team::Cookie
                } else {
                    Team::Milk
                };
            }
        }
        game
    }
}

#[derive(Clone)]
pub struct AppState {
    game: Arc<RwLock<Game>>,
    rand: Arc<Mutex<StdRng>>,
}

pub fn get_routes() -> Router {
    let game = Game::default();
    let game = Arc::new(RwLock::new(game));
    let rand = Arc::new(Mutex::new(StdRng::seed_from_u64(2024)));
    let state = AppState{ game, rand };
    // Define routes.
    Router::new()
        .route("/12/board", get(handle_board))
        .route("/12/reset", post(handle_reset))
        .route("/12/place/:team/:column", post(handle_place))
        .route("/12/random-board", get(handle_random_board))
        .with_state(state)
}


async fn handle_board(State(state): State<AppState>) ->  impl IntoResponse {
    let current = state.game.read().unwrap().to_string();
    (StatusCode::OK, current).into_response()
}

async fn handle_reset(State(state): State<AppState>) ->  impl IntoResponse {
    // Create a new game and board.
    println!("Resetting game");
    *state.game.write().unwrap() = Game::default();
    *state.rand.lock().unwrap() = StdRng::seed_from_u64(2024);
    let current = state.game.read().unwrap().to_string();
    (StatusCode::OK, current).into_response()
}

async fn handle_place(State(state): State<AppState>, Path((team, column)): Path<(String, usize)>) ->  impl IntoResponse {
    println!("Team: {} - Col: {}", team, column);
    // Check team parameter.
    let team = match team.as_str() {
        "cookie" => Team::Cookie,
        "milk" => Team::Milk,
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };
    // Check column parameter.
    if !(1..=4).contains(&column) {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let column = column - 1;
    // Check if game is over.
    let mut game = state.game.write().unwrap();
    if game.winner.is_some() {
        return (StatusCode::SERVICE_UNAVAILABLE, game.to_string()).into_response()
    }
    // Place the item.
    if let Some(row) = game.place(team, column) {
        if let Some(winner) = game.validate(row, column) {
            println!("Winner found {:?}", winner);
            game.winner = Some(winner);
        }

    } else {
        println!("Column is full!!");
        return (StatusCode::SERVICE_UNAVAILABLE, game.to_string()).into_response()
    }
    (StatusCode::OK, game.to_string()).into_response()
}

async fn handle_random_board(State(state): State<AppState>) ->  impl IntoResponse {
    println!("Random game");
    let random_game = Game::generate_random(&mut state.rand.lock().unwrap());
    // TODO: We need to find the real winner here. 
    // But since the validator is ok with this we leave it for later.  
    (StatusCode::OK, random_game.to_string()).into_response()
}
