mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaAppVersionRow(ObjectSubclass<imp::ArgenteaAppVersionRow>)
    @extends adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaAppVersionRow {
    pub fn new(version: &str, date: &str, developement: bool) -> Self {
        Object::builder()
            .property("title", version)
            .property("subtitle", date)
            .property("developement", developement)
            .build()
    }

    pub fn setup_binds(&self) {
        self.bind_property("developement", &self.imp().developement_tag.get(), "visible")
            .sync_create()
            .build();
    }
}
