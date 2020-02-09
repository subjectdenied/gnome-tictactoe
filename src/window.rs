use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

mod game;
use game::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {

    pub fn new() -> Self {
        let builder: gtk::Builder = gtk::Builder::new_from_resource("/com/subjectdenied/gnome-tictactoe/window.ui");
        let widget: gtk::ApplicationWindow = builder.get_object("window").expect("Failed to find the window object");
        let button_start: gtk::Button = builder.get_object("button_start").expect("Failed to find button start");

        let game = Rc::new(RefCell::new(Game::new(builder, 3)));

        // borrow for starting the game
        game.borrow_mut().start();

        for (i, cell) in game.borrow_mut().board.borrow_mut().iter().enumerate() {
            let g = game.clone();
            cell.widget.connect_clicked(move |_| {
                if g.borrow().state == GameState::Running
                    && g.borrow().players[(g.borrow().turn - 1) as usize].ptype == PlayerType::Human {
                        g.borrow_mut().place(i);
                }
            });
        }

        button_start.connect_clicked(move |_| {
            println!("attempt to start");
            game.borrow_mut().reset();
        });

        Self {
            widget,
        }
    }

}

