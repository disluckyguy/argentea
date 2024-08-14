mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaDescriptionBox(ObjectSubclass<imp::ArgenteaDescriptionBox>)
    @extends gtk::Box, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaDescriptionBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
