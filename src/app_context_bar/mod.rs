mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaAppContextBar(ObjectSubclass<imp::ArgenteaAppContextBar>)
    @extends gtk::Box, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaAppContextBar {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
