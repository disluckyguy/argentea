use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::cell::{Cell, RefCell};
// use glib::properties;

#[derive(CompositeTemplate, Default, glib::Properties)]
#[properties(wrapper_type = super::ArgenteaLozenge)]
#[template(file = "src/ui/lozenge.blp")]
pub struct ArgenteaLozenge {
    #[template_child]
    pub image: TemplateChild<gtk::Image>,
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    #[property(get, set)]
    circular: Cell<bool>,
    #[property(get, set, nullable)]
    icon_name: RefCell<Option<String>>,
    #[property(get, set)]
    pixel_size: Cell<i32>,
    #[property(get, set)]
    text: RefCell<String>,
    #[property(get, set)]
    markup: RefCell<String>,

}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaLozenge {
    const NAME: &'static str = "ArgenteaLozenge";
    type Type = super::ArgenteaLozenge;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for ArgenteaLozenge {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.setup_binds();
    }
}

impl WidgetImpl for ArgenteaLozenge {}

impl BoxImpl for ArgenteaLozenge {}
