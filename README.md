# Solitaire Chess Puzzle Generator

Goal: Generate 'hard' puzzles.

### Play a demo of the game [here](https://games.neophyte.me/sol_chess/)

## Install

- Install Rust from [here](https://www.rust-lang.org/tools/install).
- Run `cargo install --git https://github.com/cool-mist/sol_chess` to install the tool.
- This installs 2 binaries: `sol_chess` and `sol_cli`.

## Usage

- Run `sol_chess` to start a windowed GUI game.
- Run `sol_cli` to start the CLI tool.

## Demo site

- Play a demo of the game [here](https://games.neophyte.me/sol_chess/)

## CLI Usage

- Generate a puzzle

```bash
$ sol_cli -g -n 6
Generating a puzzle with 6 pieces with a maximum of 5 solutions
                Total attempts:     7
           Total pieces placed:    71
         Success pieces placed:    42
               Total time (ms):    69

               ♘  .  .  .

               ♙  .  ♖  .

               ♔  .  ♘  ♙

               .  .  .  .


          id: 202859896274992
```

- Solve a puzzle by ID, or by board string

```bash
$ sol_cli --solve 202859896274992
$ sol_cli --solve-board N...P.R.K.NP....

               ♘  .  .  .

               ♙  .  ♖  .

               ♔  .  ♘  ♙

               .  .  .  .


          id: 202859896274992

Found 3 solutions
1. RxNc2
2. RxPd2
3. RxKa2
4. RxPa3
5. RxNa4

```

## Heuristics of current algorithm

1. About 6-7 pieces on the board.
2. Select pieces to place based on its weight.
    3. Eg: Queen is too powerful, so it has lower weightage.
    4. Eg: Knights are confusing. More knights.

