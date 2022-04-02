use std::io;

pub enum Status {
    Go(u8),
    Position(String, Vec<String>),
    Continue,
    Quit,
}

pub struct Memory {
    pub pos: String,
    pub moves: Vec<String>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            pos: "".to_string(),
            moves: vec![],
        }
    }
}

fn identify() {
    println!("id name Weakfish");
    println!("id author juliohq");
}

fn options() {
    println!("option name Level type spin default 1 min 1 max 20");
}

fn position(fen: String, moves: Vec<String>, mem: &mut Memory) {
    mem.pos = fen;
    mem.moves = moves;
}

pub fn best_move(move_str: String) {
    println!("bestmove {}", move_str);
}

fn unknown_command(command: &str) {
    println!("Unknown command: {}", command);
}

fn unknown_parameter(p: &str) {
    println!("Unknown parameter: {}", p);
}

pub fn parse(input: String, mem: &mut Memory) -> Status {
    match input.as_str() {
        "uci\n" => {
            identify();
            options();
            println!("uciok");
        },
        "isready\n" => {
            println!("readyok");
        },
        "ucinewgame\n" => {
            
        },
        _ => {
            // Split command if it has arguments
            let split: Vec<&str> = input.split_whitespace().collect();
            let command: &str;
            if let Some(c) = split.get(0) {
                command = c;
            } else {
                return Status::Continue;
            }
            
            if command == "go" {
                let depth: u8 = 6;
                // if let Some(d_p) = split.get(2) {
                //     if d_p == &"depth" {
                //         if let Some(d) = split.get(3) {
                //             depth = d.parse().expect("Failed");
                //         }
                //     }
                // }
                
                return Status::Go(depth);
            }
            
            let param: &str;
            if let Some(p) = split.get(1) {
                param = p;
            } else {
                return Status::Continue;
            }
            
            match command.as_ref() {
                "position" => {
                    if param == "fen" {
                        let mut fen = "".to_string();
                        let mut moves: Vec<String> = vec![];
                        
                        for (i, m) in split.iter().enumerate() {
                            if i > 1 && i < 8 {
                                fen = if fen == "" { m.to_string() } else { format!("{} {}", fen, m) };
                            }
                        }
                        
                        for (i, m) in split.iter().enumerate() {
                            if i < 9 {
                                continue;
                            }
                            moves.push(m.to_string());
                        }
                        
                        position(fen.to_string(), moves.clone(), mem);
                        return Status::Position(fen.to_string(), moves);
                    } else if param == "startpos" {
                        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
                        let mut moves: Vec<String> = vec![];
                        
                        for (i, m) in split.iter().enumerate() {
                            if i < 2 {
                                continue;
                            }
                            moves.push(m.to_string());
                        }
                        
                        position(fen.to_string(), moves.clone(), mem);
                        return Status::Position(fen.to_string(), moves);
                    } else {
                        unknown_parameter(param);
                    }
                },
                _ => {
                    unknown_command(command);
                }
            }
        },
    }
    
    return Status::Continue;
}

pub fn get_input() -> String {
    let mut buffer = String::new();
    
    io::stdin()
        .read_line(&mut buffer)
        .expect("Invalid input");
    
    buffer
}