
use mrn_canvas::ItemCanvas;
use relm4::{RelmApp};

fn main() {
    let app = RelmApp::new("relm4.test.components");
    app.run::<ItemCanvas>(());
}