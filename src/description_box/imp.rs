use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use crate::lozenge::ArgenteaLozenge;
// use glib::properties;

#[derive(CompositeTemplate, Default)]
#[template(file = "src/ui/description-box.blp")]
pub struct ArgenteaDescriptionBox {
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaDescriptionBox {
    const NAME: &'static str = "ArgenteaDescriptionBox";
    type Type = super::ArgenteaDescriptionBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("app-context-bar");
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for ArgenteaDescriptionBox {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArgenteaDescriptionBox {}

impl BoxImpl for ArgenteaDescriptionBox {}
