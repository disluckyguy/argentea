use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use crate::description_box::ArgenteaDescriptionBox;
// use glib::properties;

#[derive(CompositeTemplate, Default)]
#[template(file = "src/ui/app-version-history-row.blp")]
pub struct ArgenteaAppVersionHistoryRow {
    #[template_child]
    version_description_box: TemplateChild<ArgenteaDescriptionBox>,
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaAppVersionHistoryRow {
    const NAME: &'static str = "ArgenteaAppVersionHistoryRow";
    type Type = super::ArgenteaAppVersionHistoryRow;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for ArgenteaAppVersionHistoryRow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArgenteaAppVersionHistoryRow {}

impl ListBoxRowImpl for ArgenteaAppVersionHistoryRow {}
