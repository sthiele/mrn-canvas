extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate mrn_canvas;

use gtk::{Button, ButtonExt, ContainerExt, Inhibit, WidgetExt, Window, WindowType};
use gtk::Orientation::{Horizontal, Vertical};
use relm::{Component, ContainerWidget, Relm, Update, Widget};

use mrn_canvas::MRNWidget;
use mrn_canvas::MRNWidgetMsg;

use self::Msg::*;


#[derive(Msg)]
enum Msg {
    Quit,
    Add,
    Del,
}

struct Win {
    _mrnwidget: Component<MRNWidget>,
    window: Window,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> () {
        ()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Add => {
                self._mrnwidget.emit(MRNWidgetMsg::Add);
            }
            Del => {
                self._mrnwidget.emit(MRNWidgetMsg::Del);
            }
            Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: ()) -> Win {
        let window = Window::new(WindowType::Toplevel);
        let vbox = gtk::Box::new(Vertical, 0);
        let hbox = gtk::Box::new(Horizontal, 0);
        let plus_button = Button::new_with_label("add");
        hbox.add(&plus_button);
        let minus_button = Button::new_with_label("del");
        hbox.add(&minus_button);

        let mrn = ();
        let mrnwidget = vbox.add_widget::<MRNWidget, _>(relm, mrn);

        vbox.add(&hbox);
        window.add(&vbox);
        window.show_all();


        connect!(relm, plus_button, connect_clicked(_), Add);
        connect!(relm, minus_button, connect_clicked(_), Del);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Quit), Inhibit(false)));

        Win {
            _mrnwidget: mrnwidget,
            window: window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
