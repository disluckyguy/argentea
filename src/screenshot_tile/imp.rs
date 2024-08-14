use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
// use std::cell::{Cell, RefCell};
// use glib::properties;

#[derive(CompositeTemplate, Default)]
// #[properties(wrapper_type = super::ArgenteaScreenshotTile)]
#[template(file = "src/ui/screenshot-tile.blp")]
pub struct ArgenteaScreenshotTile {

}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaScreenshotTile {
    const NAME: &'static str = "ArgenteaScreenshotTile";
    type Type = super::ArgenteaScreenshotTile;
    type ParentType = gtk::FlowBoxChild;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for ArgenteaScreenshotTile {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArgenteaScreenshotTile {}

impl FlowBoxChildImpl for ArgenteaScreenshotTile {}
