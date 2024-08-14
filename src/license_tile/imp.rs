use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
// use std::cell::{Cell, RefCell};
use crate::lozenge::ArgenteaLozenge;
// use glib::properties;

#[derive(CompositeTemplate, Default)]
// #[properties(wrapper_type = super::ArgenteaLicenseTile)]
#[template(file = "src/ui/license-tile.blp")]
pub struct ArgenteaLicenseTile {
    #[template_child]
    pub lozenge0: TemplateChild<ArgenteaLozenge>,
    #[template_child]
    pub lozenge1: TemplateChild<ArgenteaLozenge>,
    #[template_child]
    pub lozenge2: TemplateChild<ArgenteaLozenge>,
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaLicenseTile {
    const NAME: &'static str = "ArgenteaLicenseTile";
    type Type = super::ArgenteaLicenseTile;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for ArgenteaLicenseTile {
    fn constructed(&self) {
        self.parent_constructed();

        // let obj = self.obj();
    }
}

impl WidgetImpl for ArgenteaLicenseTile {}

impl BinImpl for ArgenteaLicenseTile {}
