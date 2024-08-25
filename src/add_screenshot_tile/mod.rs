mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaAddScreenshotTile(ObjectSubclass<imp::ArgenteaAddScreenshotTile>)
    @extends gtk::FlowBoxChild, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaAddScreenshotTile {
    pub fn new() -> Self {
        Object::builder()
            .build()
    }
}
