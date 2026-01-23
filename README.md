# Solitaire Chess Puzzle Generator

Goal: Generate 'hard' puzzles.

### Play a demo of the game [here](https://games.neophyte.me/sol_chess/)

## Install

- `cargo install --git https://github.com/cool-mist/sol_chess sol_chess`
- Run `sol_chess` to start a windowed GUI game.

## Update

- `cargo install --git https://github.com/cool-mist/sol_chess sol_chess --force`

## CLI Usage

- `cargo install --git https://github.com/cool-mist/sol_chess sol_cli`
- Run `sol_cli` to start the CLI tool.

- Generate a puzzle

```bash
$ sol_cli -g -n 6
Generating a puzzle with 6 pieces with a maximum of 5 solutions
                Total attempts:    16
           Total pieces placed:   131
         Success pieces placed:    96

               ♙  ♗  .  .               

               .  .  .  .               

               ♗  ♖  .  .               

               ♙  .  .  ♙               


              id: wmgYAAAG     
```

- Solve a puzzle by ID, or by board string

```bash
$ sol_cli --solve wmgYAAAG
$ sol_cli --solve-board pb......br..p..p

               ♙  ♗  .  .               

               .  .  .  .               

               ♗  ♖  .  .               

               ♙  .  .  ♙               


              id: wmgYAAAG              

1. RxBb4
2. RxPa4
3. RxBa2
4. RxPa1
5. RxPd1
There are atleast 1 solutions to this puzzle

```
