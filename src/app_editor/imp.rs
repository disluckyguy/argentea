use adw::{gio, glib, gdk};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use sourceview5::View;
use glib::Properties;
use std::cell::RefCell;
use crate::core::tools::*;

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
    pub description_text: TemplateChild<gtk::TextView>,
    #[template_child]
    pub screenshot_box: TemplateChild<gtk::FlowBox>,
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
    pub version_history_list: TemplateChild<gtk::ListBox>,
    #[template_child]
    pub use_display_length: TemplateChild<adw::SwitchRow>,
    #[template_child]
    pub min_length: TemplateChild<adw::SpinRow>,
    #[template_child]
    pub app_color_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub app_color_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub app_color_dark_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub app_color_dark_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub app_homepage: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub app_bug_tracker: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub app_donations: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub app_translations: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub categories_box: TemplateChild<gtk::FlowBox>,
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
        klass.bind_template_callbacks();
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

#[gtk::template_callbacks]
impl ArgenteaAppEditor {
    #[template_callback]
    fn open_color_dialog(&self, _button: &gtk::Button) {
        let color_dialog = gtk::ColorDialog::builder()
            .modal(true)
            .build();
        let windows: Vec<adw::ApplicationWindow> = gtk::Window::list_toplevels()
            .into_iter()
            .filter_map(|widget| widget.downcast_ref::<adw::ApplicationWindow>().cloned())
            .filter(|window| window.is_active())
            .collect();

        let window = windows.get(0).unwrap();

        color_dialog.choose_rgba(Some(window), None, None::<&gio::Cancellable>, glib::clone!(
            #[weak(rename_to = imp)]
            self,
            move |color| {
            if let Ok(res) = color {
                imp.app_color_label.set_label(&color2hex(&res.to_string()));
                let provider = gtk::CssProvider::new();
                provider.load_from_string(&format!("
                    .app-color-button {{
                        background-color: {};
                    }}
                ", res.to_string()));


                let display = gdk::Display::default().expect("failed to get display");
                gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
        }));

    }

    #[template_callback]
    fn open_color_dialog_dark(&self, _button: &gtk::Button) {
        let color_dialog = gtk::ColorDialog::builder()
            .modal(true)
            .build();
        let windows: Vec<adw::ApplicationWindow> = gtk::Window::list_toplevels()
            .into_iter()
            .filter_map(|widget| widget.downcast_ref::<adw::ApplicationWindow>().cloned())
            .filter(|window| window.is_active())
            .collect();

        let window = windows.get(0).unwrap();

        color_dialog.choose_rgba(Some(window), None, None::<&gio::Cancellable>, glib::clone!(
            #[weak(rename_to = imp)]
            self,
            move |color| {
            if let Ok(res) = color {

                imp.app_color_dark_label.set_label(&color2hex(&res.to_string()));
                let provider = gtk::CssProvider::new();
                provider.load_from_string(&format!("
                    .app-color-button-dark {{
                        background-color: {};
                    }}
                ", res.to_string()));

                let display = gdk::Display::default().expect("failed to get display");
                gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
        }));
    }
}
