extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate cairo;
extern crate gdk;

use gtk::{ContainerExt, DrawingArea, Inhibit, ScrolledWindow, ScrolledWindowExt, Viewport,
          WidgetExt, WidgetExtManual};
use relm::{Relm, Update, Widget};

use gdk::WindowExt;
use gdk::DrawingContextExt;
use gdk::DragAction;

use gdk::Atom;

use gtk::DestDefaults;
use gtk::TargetList;

use std::iter::Iterator;

use self::MRNWidgetMsg::*;

pub struct Model {
    item_list: Vec<Item>,
}
#[derive(Clone)]
struct Item {
    coord_x: f64,
    coord_y: f64,
    width: f64,
    height: f64,
}
#[derive(Msg)]
pub enum MRNWidgetMsg {
    Add,
    Del,
    BP((f64, f64)),
    BR((f64, f64)),
    DrawWidget,
    DrawSW,
    DD((i32, i32)),
}

pub struct MRNWidget {
    da: DrawingArea,
    sw: ScrolledWindow,
    vp: Viewport,
    tl: TargetList,
    model: Model,
    start_x: Option<f64>,
    start_y: Option<f64>,
    selected: Option<usize>,
}

impl Update for MRNWidget {
    type Model = Model;
    type ModelParam = ();
    type Msg = MRNWidgetMsg;

    fn model(_: &Relm<Self>, _value: ()) -> Self::Model {
        Model { item_list: vec![] }
    }

    fn update(&mut self, event: MRNWidgetMsg) {
        match event {
            Add => {
                self.add_new_item();
            }
            Del => {
                self.delete_selected_item();
            }
            BP((x, y)) => {
                self.start_x = Some(x);
                self.start_y = Some(y);
                self.selected = self.model.item_list.iter().rposition(|ref item| {
                    (x >= item.coord_x) & (x <= item.coord_x + item.width) & (y >= item.coord_y)
                        & (y <= item.coord_y + item.height)
                });

                if self.selected.is_some() {
                    let drag_action = DragAction::all();
                    self.da.drag_begin_with_coordinates(
                        &self.tl,
                        drag_action,
                        0,
                        None,
                        x as i32,
                        y as i32,
                    );
                    println!("Drag begin");
                }
                self.draw_item_list();
            }
            BR((x, y)) => {
                println!("Button released");
                if let Some(idx) = self.selected {
                    let mut sitem = self.model.item_list.remove(idx);
                    sitem.coord_x = sitem.coord_x + (x - self.start_x.unwrap());
                    sitem.coord_y = sitem.coord_y + (y - self.start_y.unwrap());
                    self.model.item_list.insert(idx, sitem);
                }
                self.draw_item_list();
            }

            DD((x, y)) => {
                println!("Drag dropped at {},{}", x, y);
                if let Some(idx) = self.selected {
                    let mut sitem = self.model.item_list.remove(idx);
                    sitem.coord_x = sitem.coord_x + (x as f64 - self.start_x.unwrap());
                    sitem.coord_y = sitem.coord_y + (y as f64 - self.start_y.unwrap());
                    self.model.item_list.insert(idx, sitem);
                }
                self.draw_item_list();
            }
            DrawWidget => {
                self.draw_item_list();
            }

            DrawSW => {
                self.draw_item_list();
            }
        }
    }
}

impl Widget for MRNWidget {
    type Root = gtk::ScrolledWindow;

    fn root(&self) -> Self::Root {
        self.sw.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let sw = ScrolledWindow::new(None, None);
        sw.set_size_request(500, 500);
        sw.set_min_content_height(500);
        sw.set_min_content_width(500);
        sw.set_overlay_scrolling(false);

        let vp = Viewport::new(None, None);

        let da = DrawingArea::new();
        da.set_size_request(700, 700);
        da.set_tooltip_text(Some("Draw things here"));
        da.set_visible(true);
        da.set_sensitive(true);
        da.set_can_focus(true);
        da.set_vexpand(true);
        da.set_hexpand(true);
        da.activate();
        da.set_receives_default(true);
        da.add_events(
            // TODO: symbolic values instead of numberic ones (gdk/gdktypes.h) when
            // gtk-rs gets fixed; http://gtk-rs.org/docs/gdk/struct.EventMask.html
            (1 << 8) // gdk::EventMask::BUTTON_PRESS_MASK
                      | (1 << 9) // gdk::EventMask::BUTTON_RELEASE_MASK
                      | (1 << 2) // gdk::EventMask::POINTER_MOTION_MASK
                      | (1 << 23) // gdk::EventMask::SMOOTH_SCROLL_MASK
                      | (1 << 10) // gdk::EventMask::KEY_PRESS_MASK
                      | (1 << 11), // gdk::EventMask::KEY_RELEASE_MASK
        );

        let destdef = DestDefaults::all();
        let dragact = DragAction::all();

        let v = vec![];
        let tl = TargetList::new(&v);
        let atom = Atom::intern("hi");
        tl.add(&atom, 0, 0);
        da.drag_dest_set(destdef, &v, dragact);
        da.drag_dest_set_target_list(Some(&tl));

        vp.add(&da);
        sw.add(&vp);

        connect!(
            relm,
            da,
            connect_button_press_event(_s, c),
            return (BP(c.get_position()), Inhibit(false))
        );
        connect!(
            relm,
            da,
            connect_button_release_event(_s, c),
            return (BR(c.get_position()), Inhibit(false))
        );

        connect!(
            relm,
            da,
            connect_drag_drop(_s, _c, x, y, _t),
            return (DD((x, y)), Inhibit(false))
        );

        //         connect!(relm, da, connect_drag_motion(s, c,x,y,t), return (DM((x,y)),Inhibit(false)));
        //         connect!(relm, da, connect_drag_end(s, c), DE(c.list_targets()));

        connect!(
            relm,
            da,
            connect_draw(_s, _c),
            return (DrawWidget, Inhibit(false))
        );
        connect!(
            relm,
            sw,
            connect_draw(_s, _c),
            return (DrawSW, Inhibit(false))
        );

        MRNWidget {
            da: da,
            sw: sw,
            vp: vp,
            tl: tl,
            model,
            start_x: None,
            start_y: None,
            selected: None,
        }
    }
}

impl MRNWidget {
    fn draw_item_list(&mut self) {
        let window = self.da.get_window().unwrap();
        let region = window.get_clip_region().unwrap();
        let dc = window.begin_draw_frame(&region).unwrap();
        let c = dc.get_cairo_context().unwrap();

        clear_surface(&c);
        for item in self.model.item_list.iter() {
            draw_item(&c, item);
        }
        if let Some(idx) = self.selected {
            let sitem = &self.model.item_list[idx];
            draw_sitem(&c, sitem);
        }
        window.end_draw_frame(&dc);
    }

    pub fn add_new_item(&mut self) {
        println!("widget add item");

        let muff = self.vp.translate_coordinates(&self.da, 1, 1);
        let (x, y) = muff.unwrap();
        let mitem = Item {
            coord_x: x as f64,
            coord_y: y as f64,
            width: 100.0,
            height: 100.0,
        };
        self.model.item_list.push(mitem);
        self.selected = Some(self.model.item_list.len() - 1);
        self.draw_item_list();
    }

    pub fn delete_selected_item(&mut self) {
        if let Some(idx) = self.selected {
            self.model.item_list.remove(idx);
            self.selected = None;
        } else {
            println!("widget del no item selected!");
        }
        self.draw_item_list();
    }
}

fn clear_surface(c: &cairo::Context) {
    c.set_source_rgb(0.9, 0.9, 0.9);
    c.paint();
}
fn draw_item(c: &cairo::Context, item: &Item) {
    c.set_source_rgb(0.7, 0.7, 0.5);
    c.rectangle(item.coord_x, item.coord_y, item.width, item.height);
    c.fill();
}
fn draw_sitem(c: &cairo::Context, item: &Item) {
    c.set_source_rgb(0.5, 0.1, 0.5);
    c.rectangle(item.coord_x, item.coord_y, item.width, item.height);
    c.fill();
}
