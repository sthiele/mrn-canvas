use gtk::cairo::{Context, Operator};
use gtk::glib::clone;
use gtk::prelude::{DrawingAreaExt, WidgetExt};
use relm4::drawing::DrawHandler;
use relm4::gtk;
use relm4::gtk::traits::GestureDragExt;
use relm4::{ComponentParts, SimpleComponent};

#[derive(Debug)]
pub struct ItemCanvas {
    item_list: Vec<Item>,
    selected: Option<Item>,
    drag_begin: Option<(f64, f64)>,
    handler: DrawHandler,
}
#[derive(Clone, Debug)]
struct Item {
    coord_x: f64,
    coord_y: f64,
    width: f64,
    height: f64,
}
#[derive(Debug)]
pub enum CanvasInputMsg {
    Add(f64, f64),
    Del,
    DragBegin(f64, f64),
    DragUpdate(f64, f64),
    DragEnd(f64, f64),
}

#[derive(Debug)]
pub enum CanvasOutputMsg {}
pub struct CanvasWidgets {
    // _da: gtk::DrawingArea,
    // snapshot: gtk::Snapshot,
    // vp: gtk::Viewport,
}

impl SimpleComponent for ItemCanvas {
    type Input = CanvasInputMsg;

    type Output = CanvasOutputMsg;

    type InitParams = ();

    type Root = gtk::ScrolledWindow;

    type Widgets = CanvasWidgets;

    fn init_root() -> Self::Root {
        gtk::ScrolledWindow::new()
    }

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut model = ItemCanvas {
            item_list: vec![],
            selected: None,
            drag_begin: None,
            handler: DrawHandler::new().unwrap(),
        };

        let vp = gtk::Viewport::default();
        let da = gtk::DrawingArea::new();
        // let snapshot = gtk::Snapshot::new();

        model.handler.init(&da);

        vp.set_child(Some(&da));
        da.set_content_width(700);
        da.set_content_height(700);
        root.set_child(Some(&vp));

        da.set_tooltip_text(Some("Drag items here"));
        da.set_sensitive(true);

        // let builder = gtk::EventControllerMotion::builder();
        // let motion_controller = builder.build();
        // da.add_controller(&motion_controller);

        // motion_controller.connect_motion(clone!(@strong sender => move |_,x,y| {
        //     sender.input(CanvasInputMsg::Move(x,y));
        // }));

        // let builder = gtk::GestureClick::builder();
        // let click_controller = builder.build();
        // da.add_controller(&click_controller);

        // click_controller.connect_pressed(clone!(@strong sender => move |_,_,x,y| {
        //     sender.input(CanvasInputMsg::ButtonPressed(x,y));
        // }));
        // click_controller.connect_released(clone!(@strong sender => move |_,_,x,y| {
        //     sender.input(CanvasInputMsg::ButtonRelease(x,y));
        // }));

        let gcb = gtk::GestureDrag::builder();
        let controller3 = gcb.build();
        da.add_controller(&controller3);

        controller3.connect_drag_begin(clone!(@strong sender => move |_,x,y| {
            sender.input(CanvasInputMsg::DragBegin(x,y));
        }));
        controller3.connect_drag_update(clone!(@strong sender => move |_,x,y| {
            sender.input(CanvasInputMsg::DragUpdate(x,y));
        }));
        controller3.connect_drag_end(clone!(@strong sender => move |_,x,y| {
            sender.input(CanvasInputMsg::DragEnd(x,y));
        }));


        let widgets = CanvasWidgets {
            // da,
            // snapshot,
            // vp,
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: &relm4::ComponentSender<Self>) {
        match message {
            CanvasInputMsg::Add(x, y) => {
                self.add_new_item(x, y);
                self.select(x, y);
            }
            CanvasInputMsg::Del => self.selected = None,
            CanvasInputMsg::DragBegin(x, y) => {
                self.select(x, y);
                if let Some(ref mut item) = self.selected {
                    self.drag_begin = Some((item.coord_x, item.coord_y))
                }
            }
            CanvasInputMsg::DragUpdate(delta_x, delta_y) => {
                if let Some((start_x, start_y)) = self.drag_begin {
                    if let Some(ref mut item) = self.selected {
                        item.coord_x = start_x + delta_x;
                        item.coord_y = start_y + delta_y;
                    }
                }
            }
            CanvasInputMsg::DragEnd(_x, _y) => {
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
                self.selected = Some(item)
            } else {
                let new_selected = self.item_list.iter().rposition(|item| {
                    (x >= item.coord_x)
                        & (x <= item.coord_x + item.width)
                        & (y >= item.coord_y)
                        & (y <= item.coord_y + item.height)
                });

                if let Some(index) = new_selected {
                    let new_item = self.item_list.remove(index);
                    self.item_list.push(item);
                    self.selected = Some(new_item);
                }else{
                    self.item_list.push(item);
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
                self.selected = Some(self.item_list.remove(index));
            }
        }
    }

    pub fn add_new_item(&mut self, x: f64, y: f64) {
        let mitem = Item {
            coord_x: x,
            coord_y: y,
            width: 100.0,
            height: 100.0,
        };
        self.item_list.push(mitem);
    }
}

fn clear_surface(c: &Context) {
    c.set_operator(Operator::Clear);
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
