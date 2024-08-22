mod imp;
use adw::{glib, gio};
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use sourceview5::prelude::*;
use appstream::Component;
use xmltree::Element;

glib::wrapper! {
    pub struct ArgenteaAppEditor(ObjectSubclass<imp::ArgenteaAppEditor>)
    @extends adw::BreakpointBin, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaAppEditor {
    pub fn new() -> Self {
        Object::builder()
            .build()
    }

    pub fn setup_source_view(&self) {
        let style_manager = adw::StyleManager::default();
        let text_buffer = self.imp().source_view.buffer();
        let source_buffer = text_buffer
            .downcast_ref::<sourceview5::Buffer>()
            .expect("buffer must be SourceBuffer");
        let language_manager = sourceview5::LanguageManager::default();
        let language = language_manager.language("xml").expect("language not present");
        source_buffer.set_language(Some(&language));
        style_manager.bind_property("dark", source_buffer, "style-scheme")
            .transform_to(move |_, is_dark: bool| {
                let source_style_manager = sourceview5::StyleSchemeManager::default();
                if is_dark {
                    return Some(source_style_manager.scheme("Adwaita-dark").expect("failed to get scheme"));
                }
                Some(source_style_manager.scheme("Adwaita").expect("failed to get scheme"))
            })
            .sync_create()
            .build();
    }

    pub fn setup_callbacks(&self) {

        self.connect_file_notify(
            move |editor| {
                if editor.file().is_none() {
                    return;
                }
                let imp = editor.imp();
                let path = editor.file().expect("file is null").path().expect("failed to get path").to_string_lossy().to_string();
                let tree = Element::parse(std::fs::read_to_string(&path).expect("failed to read").as_bytes()).expect("failed to parse to Element");
                let component = Component::try_from(&tree).expect("failed to parse to component");
                imp.app_name.set_text(component.name.get_default().unwrap_or(&"".to_string()));
                if let Some(developer_name) = component.developer_name {
                    imp.developer_name.set_text(developer_name.get_default().unwrap_or(&"".to_string()));
                } else {
                    if let Some(developer) = tree.get_child("developer") {
                        imp.developer_name.set_text()
                    }
                }

                imp.app_summary.set_text(component.summary.unwrap_or(appstream::TranslatableString::with_default("")).get_default().unwrap_or(&"".to_string()));
                imp.contact_email.set_text(&component.update_contact.unwrap_or("".to_string()));
                imp.app_homepage.set_text("");
                imp.app_donations.set_text("");
                imp.app_translations.set_text("");
                imp.app_bug_tracker.set_text("");
                for url in &component.urls {
                    match url {
                        appstream::enums::ProjectUrl::Homepage(url) => {
                            imp.app_homepage.set_text(&url.to_string());
                        }
                        appstream::enums::ProjectUrl::Donation(url) => {
                            imp.app_donations.set_text(&url.to_string());
                        }
                        appstream::enums::ProjectUrl::Translate(url) => {
                            imp.app_translations.set_text(&url.to_string());
                        }
                        appstream::enums::ProjectUrl::BugTracker(url) => {
                            imp.app_bug_tracker.set_text(&url.to_string());
                        }
                        _ => {}
                    }
                }
                imp.metadata_license.set_selected(0);
                if let Some(metadata_license) = component.metadata_license {
                    let model = imp.metadata_license.model().expect("failed to get model");
                    let string_list = model.downcast_ref::<gtk::StringList>()
                        .expect("model must be string list");
                    for i in 0..string_list.n_items() {
                        if string_list.string(i).expect("failed to get string") == metadata_license.0 {
                            imp.metadata_license.set_selected(i);
                        }
                    }
                }
                imp.project_license.set_selected(0);
                if let Some(project_license) = component.project_license{
                    let model = imp.project_license.model().expect("failed to get model");
                    let string_list = model.downcast_ref::<gtk::StringList>()
                        .expect("model must be string list");
                    for i in 0..string_list.n_items() {
                        if string_list.string(i).expect("failed to get string") == project_license.0 {
                            imp.project_license.set_selected(i);
                        }
                    }
                }
                let path = editor.file().unwrap().path().expect("failed to get path");
                let path_str = path.to_str().unwrap_or("");
                if let Ok(icon_path) = crate::core::tools::get_icon_path(path_str) {
                    imp.icon_stack.set_visible_child_name("present");
                    imp.app_icon.set_from_file(Some(icon_path));
                } else {
                    imp.icon_stack.set_visible_child_name("empty");
                }

            }
        );
    }

    pub fn setup_binds(&self) {
        let imp = self.imp();

        self.bind_property("visible-editor", &imp.stack.get(), "visible-child-name")
            .bidirectional()
            .sync_create()
            .build();
    }
}
