mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaLicenseTile(ObjectSubclass<imp::ArgenteaLicenseTile>)
    @extends adw::Bin, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaLicenseTile {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
