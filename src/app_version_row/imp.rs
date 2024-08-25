use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use glib::Properties;
use std::cell::Cell;

#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::ArgenteaAppVersionRow)]
#[template(file = "src/ui/app-version-row.blp")]
pub struct ArgenteaAppVersionRow {
    #[property(get, set)]
    pub developement: Cell<bool>,
    #[template_child]
    pub developement_tag: TemplateChild<gtk::Box>
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaAppVersionRow {
    const NAME: &'static str = "ArgenteaAppVersionRow";
    type Type = super::ArgenteaAppVersionRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for ArgenteaAppVersionRow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.setup_binds();
    }
}

impl WidgetImpl for ArgenteaAppVersionRow {}

impl ListBoxRowImpl for ArgenteaAppVersionRow {}

impl PreferencesRowImpl for ArgenteaAppVersionRow {}

impl ActionRowImpl for ArgenteaAppVersionRow {}


