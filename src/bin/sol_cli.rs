use argh::FromArgs;

use sol_chess::board::cmove::CMove;
use sol_chess::board::Board;
use sol_chess::generator::{self, Puzzle, RandomRange};

// Learn how to specify a different dependency for this binary
struct MacroquadRngTodo;
impl RandomRange for MacroquadRngTodo {
    fn gen_range(&self, min: usize, max: usize) -> usize {
        macroquad::rand::gen_range(min, max)
    }
}

fn main() {
    let args: Args = argh::from_env();
    if args.generate {
        let puzzle = generate_puzzle(args.num_pieces, args.solutions);
        let Some(puzzle) = puzzle else {
            println!("Failed to generate a puzzle, try adjusting the generation parameters");
            return;
        };

        if args.print {
            print_solutions(puzzle);
        }

        return;
    }

    let board = if let Some(board_string) = args.solve_board {
        Board::from_string(board_string)
    } else if let Some(board_id) = args.solve {
        Board::from_id(&board_id)
    } else {
        println!("Use --help to see available options");
        return;
    };
    let Ok(board) = board else {
        println!("Invalid board string/id");
        return;
    };

    let puzzle = board.solve();
    print_solutions(puzzle);
}

fn print_solutions(puzzle: Puzzle) {
    puzzle.board.pretty_print();
    let solutions = puzzle.solutions;
    if solutions.len() == 0 {
        println!("No solutions found");
        return;
    }
    let solution = solutions.first().unwrap();
    let mut idx = 0;
    solution.iter().for_each(|m| {
        idx += 1;
        println!("{}. {}", idx, m.notation());
    });
    println!(
        "There are atleast {} solutions to this puzzle",
        solutions.len()
    );
}

fn generate_puzzle(num_pieces: Option<u32>, num_solutions: Option<u32>) -> Option<Puzzle> {
    let mut num_pieces = num_pieces.unwrap_or(5);
    if num_pieces < 2 {
        num_pieces = 2;
    }

    let mut num_solutions = num_solutions.unwrap_or(5);
    if num_solutions < 1 {
        num_solutions = 5;
    }

    println!(
        "Generating a puzzle with {} pieces with a maximum of {} solutions",
        num_pieces, num_solutions
    );
    let gen = generator::generate(num_pieces, num_solutions, &MacroquadRngTodo);
    gen.print_stats();

    let Some(puzzle) = gen.puzzle() else {
        println!("Failed to generate a puzzle, try again");
        return None;
    };

    Some(puzzle)
}

/// Solitaire Chess puzzle generator and solver
/// - v0.0.1 cool-mist
#[derive(FromArgs)]
struct Args {
    #[argh(switch, short = 'g')]
    /// generate a puzzle
    generate: bool,

    #[argh(option, short = 'n')]
    /// number of pieces to place on the board while generating a puzzle
    num_pieces: Option<u32>,

    #[argh(option)]
    /// maximum number of solutions allowed for the generated puzzle. atleast 1. defaults to 5
    solutions: Option<u32>,

    #[argh(switch)]
    /// print the solution. When solving a puzzle, this is always set to true
    print: bool,

    #[argh(option, short = 's')]
    /// the id of the board to solve
    solve: Option<String>,

    #[argh(option)]
    /// the board to solve in board representation
    solve_board: Option<String>,
}
