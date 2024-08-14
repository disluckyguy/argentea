use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use crate::lozenge::ArgenteaLozenge;
// use glib::properties;

#[derive(CompositeTemplate, Default)]
#[template(file = "src/ui/app-context-bar.blp")]
pub struct ArgenteaAppContextBar {
    #[template_child]
    safety_tile_lozenge: TemplateChild<ArgenteaLozenge>,
    #[template_child]
    storage_tile_lozenge: TemplateChild<ArgenteaLozenge>,
    #[template_child]
    hardware_support_tile_lozenge: TemplateChild<ArgenteaLozenge>,
    #[template_child]
    age_rating_tile_lozenge: TemplateChild<ArgenteaLozenge>,
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaAppContextBar {
    const NAME: &'static str = "ArgenteaAppContextBar";
    type Type = super::ArgenteaAppContextBar;
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
impl ObjectImpl for ArgenteaAppContextBar {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArgenteaAppContextBar {}

impl BoxImpl for ArgenteaAppContextBar {}
