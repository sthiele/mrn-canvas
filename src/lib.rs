// use gdk::DragAction;
// use gdk::DrawingContextExt;
// use gdk::WindowExt;
// use gtk::DestDefaults;
// use gtk::TargetList;
use gtk::cairo::Context;
use gtk::glib::clone;
use relm4::gtk;
use relm4::gtk::traits::GestureDragExt;
use relm4::{ComponentParts, SimpleComponent};
use gtk::prelude::{DrawingAreaExt, GtkWindowExt, WidgetExt};
use relm4::drawing::DrawHandler;
pub struct ItemCanvas {
    item_list: Vec<Item>,
    selected: Option<Item>,
    drag_begin: Option<(f64, f64)>,
    handler: DrawHandler,
}
#[derive(Clone)]
struct Item {
    coord_x: f64,
    coord_y: f64,
    width: f64,
    height: f64,
}
#[derive(Debug)]
pub enum CanvasWidgetsMsg {
    ButtonPressed((f64, f64)),
    ButtonRelease((f64, f64)),
    Move((f64, f64)),
    DragBegin((f64, f64)),
    DragUpdate((f64, f64)),
    DragEnd((f64, f64)),
}

pub struct CanvasWidgets {
    da: gtk::DrawingArea,
    snapshot: gtk::Snapshot,
    sw: gtk::ScrolledWindow,
    vp: gtk::Viewport,
}

impl SimpleComponent for ItemCanvas {
    type Input = CanvasWidgetsMsg;

    type Output = ();

    type InitParams = ();
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;

    type Widgets = CanvasWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Simple app")
            .default_width(300)
            .default_height(100)
            .build()
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut model = ItemCanvas {
            item_list: vec![],
            selected: None,
            drag_begin: None,
            handler: DrawHandler::new().unwrap(),
        };
        let sw = gtk::ScrolledWindow::new();
        sw.set_overlay_scrolling(false);

        let vp = gtk::Viewport::default();
        let da = gtk::DrawingArea::new();
        let snapshot = gtk::Snapshot::new();

        model.handler.init(&da);

        vp.set_child(Some(&da));
        sw.set_child(Some(&vp));
        da.set_content_width(700);
        da.set_content_height(700);
        root.set_child(Some(&sw));

        da.set_tooltip_text(Some("Drag items here"));
        da.set_sensitive(true);

        let gcb = gtk::EventControllerMotion::builder();
        let controller = gcb.build();
        da.add_controller(&controller);

        controller.connect_motion(clone!(@strong sender => move |_,x,y| {
            sender.input(CanvasWidgetsMsg::Move((x,y)));
        }));

        let gcb = gtk::GestureClick::builder();
        let controller2 = gcb.build();
        da.add_controller(&controller2);

        controller2.connect_pressed(clone!(@strong sender => move |_,_,x,y| {
            sender.input(CanvasWidgetsMsg::ButtonPressed((x,y)));
        }));
        controller2.connect_released(clone!(@strong sender => move |_,_,x,y| {
            sender.input(CanvasWidgetsMsg::ButtonRelease((x,y)));
        }));

        let gcb = gtk::GestureDrag::builder();
        let controller3 = gcb.build();
        da.add_controller(&controller3);

        controller3.connect_drag_begin(clone!(@strong sender => move |_,x,y| {
            sender.input(CanvasWidgetsMsg::DragBegin((x,y)));
        }));
        controller3.connect_drag_update(clone!(@strong sender => move |_,x,y| {
            sender.input(CanvasWidgetsMsg::DragUpdate((x,y)));
        }));
        controller3.connect_drag_end(clone!(@strong sender => move |_,x,y| {
            sender.input(CanvasWidgetsMsg::DragEnd((x,y)));
        }));

        model.add_new_item();
        model.add_new_item();
        model.add_new_item();

        let widgets = CanvasWidgets {
            da,
            snapshot,
            sw,
            vp,
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: &relm4::ComponentSender<Self>) {
        match message {
            CanvasWidgetsMsg::ButtonPressed((x, y)) => {
                self.select(x, y);
            }
            CanvasWidgetsMsg::ButtonRelease((x, y)) => {
                println!("Button released {x} {y}");
            }
            CanvasWidgetsMsg::Move(_) => {
            }
            CanvasWidgetsMsg::DragBegin((x, y)) => {
                self.select(x, y);
                if let Some(ref mut item) = self.selected {
                    self.drag_begin = Some((item.coord_x, item.coord_y))
                }
            }
            CanvasWidgetsMsg::DragUpdate((delta_x, delta_y)) => {
                println!("Drag pos {delta_x} {delta_y}");
                if let Some((start_x, start_y)) = self.drag_begin {
                    if let Some(ref mut item) = self.selected {
                        item.coord_x = start_x + delta_x;
                        item.coord_y = start_y + delta_y;
                    }
                }
            }
            CanvasWidgetsMsg::DragEnd((x, y)) => {
                println!("Drag end at {x} {y}");
                self.drag_begin = None;
            }
        }

        let cx = self.handler.get_context().unwrap();

        clear_surface(&cx);

        let cx = self.handler.get_context().unwrap();
        for item in &self.item_list {
            draw_item(&cx, item);
        }
        if let Some(ref item) = self.selected {
            draw_sitem(&cx, item);
        }
    }
}

impl ItemCanvas {
    fn select(&mut self, x: f64, y: f64) {
        if let Some(item) = self.selected.take() {
            if (x >= item.coord_x)
                & (x <= item.coord_x + item.width)
                & (y >= item.coord_y)
                & (y <= item.coord_y + item.height)
            {
                println!("SAME SELECTED ");
                self.selected = Some(item)
            } else {
                let new_selected = self.item_list.iter().rposition(|item| {
                    (x >= item.coord_x)
                        & (x <= item.coord_x + item.width)
                        & (y >= item.coord_y)
                        & (y <= item.coord_y + item.height)
                });

                if let Some(index) = new_selected {
                    println!("NEW SELECTED {}", index);
                    let new_item = self.item_list.remove(index);
                    self.item_list.push(item);
                    self.selected = Some(new_item);
                } 
            }
        } else {
            let selected = self.item_list.iter().rposition(|item| {
                (x >= item.coord_x)
                    & (x <= item.coord_x + item.width)
                    & (y >= item.coord_y)
                    & (y <= item.coord_y + item.height)
            });

            if let Some(index) = selected {
                println!("NEW SELECTED {}", index);
                self.selected = Some(self.item_list.remove(index));
            } 
        }
    }

    pub fn add_new_item(&mut self) {
        println!("widget add item");

        let mitem = Item {
            coord_x: f64::from(1),
            coord_y: f64::from(1),
            width: 100.0,
            height: 100.0,
        };
        self.item_list.push(mitem);
    }

    //     pub fn delete_selected_item(&mut self) {
    //         if let Some(idx) = self.selected {
    //             self.model.item_list.remove(idx);
    //             self.selected = None;
    //         } else {
    //             println!("widget del no item selected!");
    //         }
    //         self.draw_item_list();
    //     }
}

fn clear_surface(c: &Context) {
    // c.set_operator(Operator::Clear);
    c.set_source_rgb(0.9, 0.9, 0.9);
    c.paint().unwrap();
}
fn draw_item(c: &Context, item: &Item) {
    c.set_source_rgb(0.7, 0.7, 0.5);
    c.rectangle(item.coord_x, item.coord_y, item.width, item.height);
    c.fill().unwrap();
}
fn draw_sitem(c: &Context, item: &Item) {
    draw_item(c, item);
    c.set_source_rgb(0.5, 0.1, 0.5);
    c.rectangle(item.coord_x, item.coord_y, item.width, item.height);
    c.stroke().unwrap();
}
