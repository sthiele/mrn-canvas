use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use mrn_canvas::CanvasInputMsg;
use mrn_canvas::ItemCanvas;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent, WidgetPlus,
};
struct AppModel {
    canvas: Controller<ItemCanvas>,
}

#[derive(Debug)]
enum AppInput {
    Add,
    Del,
}

struct AppWidgets {}

impl SimpleComponent for AppModel {
    /// The type of the messages that this component can receive.
    type Input = AppInput;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data that this component will be initialized with.
    type InitParams = ();
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Demo item canvas")
            .default_width(500)
            .default_height(500)
            .build()
    }

    /// Initialize the UI and model.
    fn init(
        _params: Self::InitParams,
        window: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let builder = ItemCanvas::builder();
        let connector = builder.launch(());
        let canvas = connector.forward(&sender.input, |msg| match msg {});
        let model = AppModel { canvas };

        let item_canvas = model.canvas.widget();
        item_canvas.set_overlay_scrolling(false);

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let add_button = gtk::Button::with_label("Add");
        let del_button = gtk::Button::with_label("Del");

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);

        vbox.append(item_canvas);
        vbox.append(&add_button);
        vbox.append(&del_button);
        item_canvas.set_propagate_natural_height(true);
        add_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Add);
        }));

        del_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Del);
        }));

        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: &relm4::ComponentSender<Self>) {
        match message {
            AppInput::Add => {
                self.canvas.emit(CanvasInputMsg::Add(1.0, 1.0));
            }
            AppInput::Del => {
                self.canvas.emit(CanvasInputMsg::Del);
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: &ComponentSender<Self>) {}
}

fn main() {
    let app: RelmApp = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(());
}
