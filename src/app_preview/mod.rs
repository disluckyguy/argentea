mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaAppPreview(ObjectSubclass<imp::ArgenteaAppPreview>)
    @extends adw::BreakpointBin, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaAppPreview {
    pub fn new() -> Self {
        Object::builder()
            .build()
    }
}
