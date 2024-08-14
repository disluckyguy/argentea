mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaAppVersionHistoryRow(ObjectSubclass<imp::ArgenteaAppVersionHistoryRow>)
    @extends gtk::ListBoxRow, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaAppVersionHistoryRow {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
