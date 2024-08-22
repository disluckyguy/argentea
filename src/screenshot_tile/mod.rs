mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaScreenshotTile(ObjectSubclass<imp::ArgenteaScreenshotTile>)
    @extends gtk::FlowBoxChild, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaScreenshotTile {
    pub fn new(uri: &str) -> Self {
        Object::builder()
            .property("uri", uri)
            .build()
    }
}
