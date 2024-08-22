use adw::{gio, glib};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use sourceview5::View;
use glib::Properties;
use std::cell::RefCell;

#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::ArgenteaAppEditor)]
#[template(file = "src/ui/app-editor.blp")]
pub struct ArgenteaAppEditor {
    #[property(get, set, construct, nullable)]
    pub file: RefCell<Option<gio::File>>,
    #[property(get, set, default = "gui-editor", construct)]
    pub visible_editor: RefCell<String>,
    #[template_child]
    pub source_view: TemplateChild<View>,
    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub icon_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub app_icon: TemplateChild<gtk::Image>,
    #[template_child]
    pub app_name: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub developer_name: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub app_summary: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub contact_email: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub keyboard_support: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub mouse_support: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub touchscreen_support: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub gamepad_support: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub graphics_tablet_support: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub app_homepage: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub app_bug_tracker: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub app_donations: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub app_translations: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub project_license: TemplateChild<adw::ComboRow>,
    #[template_child]
    pub metadata_license: TemplateChild<adw::ComboRow>,

}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaAppEditor {
    const NAME: &'static str = "ArgenteaAppEditor";
    type Type = super::ArgenteaAppEditor;
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
impl ObjectImpl for ArgenteaAppEditor {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.setup_source_view();
        obj.setup_binds();
        obj.setup_callbacks();
    }
}

impl WidgetImpl for ArgenteaAppEditor {}

impl BreakpointBinImpl for ArgenteaAppEditor {}
