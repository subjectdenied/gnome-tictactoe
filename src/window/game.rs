use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Running,
    Won,
    Deuce,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerType {
    Human,
    AI
}

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    pub ptype: PlayerType,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub widget: gtk::Button,
    value: u8,
}

#[derive(Clone, Debug)]
pub struct Game {
    builder: gtk::Builder,
    size: u8,
    pub board: Rc<RefCell<Vec<Cell>>>,
    pub players: Vec<Player>,
    pub turn: u8,
    turns_left: u8,
    pub state: GameState,
}

impl Game {
    pub fn new(builder: gtk::Builder, size: u8) -> Self {
        Game {
            builder,
            size,
            board: Rc::new(RefCell::new(Vec::new())),
            players: vec![
                Player {
                    name: "p1".to_string(),
                    ptype: PlayerType::Human,
                },
                Player {
                    name: "p2".to_string(),
                    ptype: PlayerType::Human,
                }
            ],
            turn: 1,
            turns_left: size * size,
            state: GameState::Running,
        }
    }

    pub fn start(&mut self) {
        self.set_board_size(self.size);
        self.state = GameState::Running;
        self.clone().render();
    }

    pub fn set_board_size(&mut self, size: u8) {
        let mut board: Vec<Cell> = Vec::new();
        let length = (size * size) as usize;

        println!("length: {}", length);

        for i in 0..length {
            println!("cell number: {}", i);
            let cell = Cell {
                widget: self.builder.get_object(format!("item_{}", (i as u8) + 1).as_str()).expect("item not found"),
                value: 0,
            };

            board.push(cell);
        }

        self.board = Rc::new(RefCell::new(board));
    }

    pub fn reset(&mut self) {
        let length = self.board.borrow().len();
        for i in 0..length {
            self.board.borrow_mut()[i].value = 0;
        }

        self.turns_left = self.size * self.size;
        self.turn = 1;
        self.state = GameState::Running;

        self.clone().render();
    }

    pub fn check (&self, pos: usize) -> bool {
        // if self.board.borrow_mut()[pos].value !=
        let mut solved = true;
        let pos = pos as u8;
        let x: u8 = pos % self.size;
        let y: u8 = pos / self.size;
        println!("pos: {}, x: {}, y: {}", pos, x, y);

        // row
        let first_in_row = y * self.size;
        for i in first_in_row..(first_in_row + self.size) {
            // println!("row checking: {}", i);
            if self.board.borrow()[i as usize].value != self.turn {
                solved = false;
                break;
            }
        }

        // col
        if !solved {
            solved = true;
            let mut n = 0;

            while n < self.size {
                let pos = x + self.size * n;
                // println!("col checking: {}", pos);
                if self.board.borrow()[pos as usize].value != self.turn {
                    solved = false;
                    break;
                }
                n += 1;
            }
        }

        // cross top-left to bottom-right
        if !solved {
            solved = true;

            let mut n = 0;

            while n < self.size {
                let pos = (0 + self.size * n) + (n * 1);
                println!("tl-br checking: {}", pos);
                if self.board.borrow()[pos as usize].value != self.turn {
                    solved = false;
                    break;
                }
                n += 1;
            }
        }

        // cross top-right to bottom-left
        if !solved {
            solved = true;

            let last = self.size - 1;
            let mut n = 0;

            while n < self.size {
                let pos = (last + self.size * n) - (n * 1);
                println!("tr-bl checking: {}", pos);
                if self.board.borrow()[pos as usize].value != self.turn {
                    solved = false;
                    break;
                }
                n += 1;
            }
        }

        solved
    }

    pub fn place (&mut self, pos: usize) {
        // println!("{:?}", self.board.borrow_mut());

        println!("set for {} at {}", self.turn, pos);
        self.board.borrow_mut()[pos].value = self.turn;

        self.turns_left -= 1;
        println!("turns left: {}", self.turns_left);

        if self.check(pos) {
            println!("{} wins", self.players[(self.turn - 1) as usize].name);
            self.state = GameState::Won;

        } else if self.turns_left == 0 {
            println!("its a duce");
            self.state = GameState::Deuce;
        } else {
            println!("self.turn: {}", self.turn);
            self.turn = (self.turn % 2) + 1;
            println!("next turn: {}", self.turn);
        }

        self.clone().render();
    }

    pub fn render(&mut self) {
        let label_turn: gtk::Label = self.builder.get_object("label_turn").expect("item not found");
        let colors = vec!["green", "red"];

        match self.state {
            GameState::Running => {
                label_turn.set_markup(format!("<span foreground=\"{}\">{} - it's your turn</span>", colors[(self.turn as usize) - 1], self.players[(self.turn as usize) - 1].name).as_str());
            },
            GameState::Won => {
                label_turn.set_markup(format!("<span foreground=\"{}\"><big>{} wins the game!</big></span>", colors[(self.turn as usize) - 1], self.players[(self.turn as usize) - 1].name).as_str());
            },
            GameState::Deuce => {
                label_turn.set_markup(format!("<span foreground=\"black\"><big>it's a deuce!</big></span>").as_str());
            }
        }

        for cell in self.board.borrow_mut().clone().iter() {

            match cell.value {
                0 => {
                        cell.widget.set_label("");
                        cell.widget.set_sensitive(true);
                        let style = WidgetExt::get_style_context(&cell.widget.get_child().unwrap());
                        style.add_class("turn0");
                        style.remove_class("turn1");
                        style.remove_class("turn2");
                },
                1 => {
                        cell.widget.set_label(format!("O").as_str());
                        cell.widget.set_sensitive(false);
                        let style = WidgetExt::get_style_context(&cell.widget.get_child().unwrap());
                        style.add_class("turn1");
                        style.remove_class("turn0");
                        style.remove_class("turn2");
                },
                2 => {

                        cell.widget.set_label(format!("X").as_str());
                        cell.widget.set_sensitive(false);
                        let style = WidgetExt::get_style_context(&cell.widget.get_child().unwrap());
                        style.add_class("turn2");
                        style.remove_class("turn0");
                        style.remove_class("turn1");
                },
                _ => println!("should not happen"),
            }
        }
    }

    pub fn set_players() {}
}

