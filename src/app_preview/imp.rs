use adw::{glib, gio};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use crate::app_context_bar::ArgenteaAppContextBar;
use crate::description_box::ArgenteaDescriptionBox;
use crate::app_version_history_row::ArgenteaAppVersionHistoryRow;
use crate::license_tile::ArgenteaLicenseTile;
use glib::Properties;
use std::cell::RefCell;

#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::ArgenteaAppPreview)]
#[template(file = "src/ui/app-preview.blp")]
pub struct ArgenteaAppPreview {
    #[property(get, set, construct, nullable)]
    pub file: RefCell<Option<gio::File>>,
    #[template_child]
    pub context_bar: TemplateChild<ArgenteaAppContextBar>,
    #[template_child]
    pub app_description_box: TemplateChild<ArgenteaDescriptionBox>,
    #[template_child]
    pub version_history_row: TemplateChild<ArgenteaAppVersionHistoryRow>,
    #[template_child]
    pub license_tile: TemplateChild<ArgenteaLicenseTile>,
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaAppPreview {
    const NAME: &'static str = "ArgenteaAppPreview";
    type Type = super::ArgenteaAppPreview;
    type ParentType = adw::BreakpointBin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("app-context-bar");
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for ArgenteaAppPreview {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArgenteaAppPreview {}

impl BreakpointBinImpl for ArgenteaAppPreview {}
