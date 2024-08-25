mod imp;
use adw::{glib, gio, gdk};
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use sourceview5::prelude::*;
use appstream::Component;
use xmltree::Element;
use crate::screenshot_tile::ArgenteaScreenshotTile;
use crate::app_version_row::ArgenteaAppVersionRow;
use crate::add_screenshot_tile::ArgenteaAddScreenshotTile;

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
        let _imp = self.imp();
        let (sender, receiver) = async_channel::bounded(1);

        self.connect_file_notify(
            move |editor| {
                if editor.file().is_none() {
                    return;
                }
                let imp = editor.imp();

                let path = editor.file().expect("file is null").path().expect("failed to get path").to_string_lossy().to_string();
                let text = std::fs::read_to_string(&path).expect("failed to read");
                let tree = Element::parse(text.as_bytes()).expect("failed to parse to Element");

                imp.source_view.buffer().set_text(&text);

                let component = Component::try_from(&tree).expect("failed to parse to component");
                imp.app_name.set_text(component.name.get_default().unwrap_or(&"".to_string()));
                if let Some(developer_name) = component.developer_name {
                    imp.developer_name.set_text(developer_name.get_default().unwrap_or(&"".to_string()));
                } else {
                    if let Some(developer) = tree.get_child("developer") {
                        let name = developer.get_child("name").map_or(None, |v| {Some(v.get_text().map_or(String::new(), |v| v.to_string()))}).unwrap_or("".to_string());
                        imp.developer_name.set_text(&name);
                    }
                }

                imp.mouse_support.set_active(false);
                imp.keyboard_support.set_active(false);
                imp.gamepad_support.set_active(false);
                imp.touchscreen_support.set_active(false);
                imp.graphics_tablet_support.set_active(false);

                if let Some(supports) = tree.get_child("supports") {
                    let controls: Vec<String> = supports
                        .clone()
                        .children
                        .into_iter()
                        .filter_map(|i| i.as_element().cloned())
                        .filter(|e| e.name == "control")
                        .filter_map(|i| i.get_text().map(|t| t.to_string()))
                        .collect();

                    for control in &controls {
                        match control.as_str() {
                            "pointing" => imp.mouse_support.set_active(true),
                            "keyboard" => imp.keyboard_support.set_active(true),
                            "gamepad" => imp.gamepad_support.set_active(true),
                            "touch" => imp.touchscreen_support.set_active(true),
                            "tablet" => imp.graphics_tablet_support.set_active(true),
                            _ => {}
                        }
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

                let sender = sender.clone();
                let path = editor.file().unwrap().path().expect("failed to get path");
                let path_str = path.to_str().map_or("".to_string(), |s| s.to_string());
                imp.icon_stack.set_visible_child_name("empty");
                gio::spawn_blocking(glib::clone!(
                    #[strong]
                    path_str,
                    move || {
                        let icon_path = crate::core::tools::get_icon_path(&path_str);
                        if let Ok(icon_path) = icon_path {
                            sender
                                .send_blocking(icon_path)
                                .expect("The channel needs to be open.");
                        }
                }));


                if let Some(description) = component.description {
                    let text = description.get_default().map_or(String::from(""), |s| s.to_string());



                    let paragraphs: Vec<String> = text.split_inclusive("</p>").map(|s| s.to_string()).collect();
                    let mut elements = Vec::new();
                    for p in &paragraphs {
                        let mut split: Vec<&str> = p.split_inclusive("</ul>").collect();
                        elements.append(&mut split);
                    }
                    let mut text_vec = Vec::new();
                    for element in &elements {
                        let element = Element::parse(element.as_bytes()).expect("failed to parse");
                        let mut buf = std::io::BufWriter::new(Vec::new());
                        let conf = xmltree::EmitterConfig::new()
                            .perform_indent(true)
                            .write_document_declaration(false);
                        element.write_with_config(&mut buf, conf).expect("failed to write");
                        let bytes = buf.into_inner().expect("failed to get inner");
                        let string = String::from_utf8(bytes).expect("failed to convert from utf8");

                        text_vec.push(string);
                    }

                    let text = text_vec.join("\n");

                    imp.description_text.buffer().set_text(&text);
                } else {
                    imp.description_text.buffer().set_text("");
                }

                imp.screenshot_box.remove_all();


                for screenshot in &component.screenshots {
                    let source_images: Vec<String> = screenshot.images
                        .clone()
                        .into_iter()
                        .filter(|i| i.kind == appstream::enums::ImageKind::Source)
                        .map(|i| i.url.to_string())
                        .collect();

                    if let Some(first) = source_images.get(0) {
                        let tile = ArgenteaScreenshotTile::new(&first);
                        imp.screenshot_box.append(&tile);
                    }
                }

                let add_screenshot = ArgenteaAddScreenshotTile::new();

                imp.screenshot_box.append(&add_screenshot);

                imp.use_display_length.set_active(false);
                imp.min_length.set_value(0f64);

                for requirement in component.requirements {
                    if let Ok(length) = requirement.0.parse::<f64>() {
                        imp.min_length.set_value(length);
                        imp.use_display_length.set_active(true);
                    }
                }

                imp.app_color_label.set_label("");
                let provider = gtk::CssProvider::new();
                provider.load_from_string("
                    .app-color-button {
                        background-color: #ffffff;
                    }
                ");

                let display = gdk::Display::default().expect("failed to get display");
                                    gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

                imp.app_color_dark_label.set_label("");
                let provider = gtk::CssProvider::new();
                provider.load_from_string("
                    .app-color-button-dark {
                        background-color: #000000;
                    }
                ");

                 let display = gdk::Display::default().expect("failed to get display");
                                    gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);


                if let Some(branding) = tree.get_child("branding") {
                    let elements: Vec<Element> = branding.clone().children.into_iter().filter_map(|v| v.as_element().cloned()).collect();

                    for element in &elements {
                        if element.attributes.get("type").map_or("", |s| s.as_str()) == "primary" {
                            if element.attributes.get("scheme_preference").map_or("", |s| s.as_str()) == "dark" {
                                if let Some(text) = element.get_text() {
                                    imp.app_color_dark_label.set_label(&text);
                                    let provider = gtk::CssProvider::new();
                                    provider.load_from_string(&format!("
                                        .app-color-button-dark {{
                                            background-color: {};
                                        }}
                                    ", text));

                                    let display = gdk::Display::default().expect("failed to get display");
                                    gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

                                }
                            } else if element.attributes.get("scheme_preference").map_or("", |s| s.as_str()) == "light" {
                                if let Some(text) = element.get_text() {
                                    imp.app_color_label.set_label(&text);
                                    let provider = gtk::CssProvider::new();
                                    provider.load_from_string(&format!("
                                        .app-color-button {{
                                            background-color: {};
                                        }}
                                    ", text));

                                    let display = gdk::Display::default().expect("failed to get display");
                                    gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
                                }
                            }
                        }
                    }
                }

               imp.version_history_list.remove_all();
                for release in &component.releases {
                    let row = ArgenteaAppVersionRow::new(
                        &release.version,
                        &release.date.map(|d| d.date_naive().to_string()).unwrap_or(String::new()),
                        release.kind == appstream::enums::ReleaseKind::Development
                    );
                    imp.version_history_list.append(&row);

                }

                let button_row = adw::ButtonRow::builder()
                        .start_icon_name("plus-large-symbolic")
                        .build();
                imp.version_history_list.append(&button_row);

                imp.categories_box.remove_all();

                if let Some(categories) = tree.get_child("categories") {
                    let categories_text : Vec<String> = categories.children
                        .clone()
                        .into_iter()
                        .filter_map(|i| i.as_element().cloned())
                        .filter_map(|e| e.get_text().map(|s| s.to_string()))
                        .collect();

                    for catagory in categories_text {
                        let label = gtk::Label::builder()
                            .label(&catagory)
                            .hexpand(true)
                            .xalign(0.5)
                            .halign(gtk::Align::Center)
                            .build();
                        let button = gtk::Button::builder()
                            .icon_name("cross-small-symbolic")
                            .css_classes(["circular", "flat", "accent"])
                            .halign(gtk::Align::End)
                            .build();

                        let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);

                        content_box.append(&label);
                        content_box.append(&button);

                        content_box.add_css_class("tag");

                        let child = gtk::FlowBoxChild::builder()
                            .child(&content_box)
                            .build();

                        button.connect_clicked(glib::clone!(
                            #[weak(rename_to = categories)]
                            imp.categories_box.get(),
                            #[weak]
                            child,
                            move |_| {
                                categories.remove(&child);
                            }
                        ));

                        imp.categories_box.append(&child);
                    }
                }
                let button_content = adw::ButtonContent::builder()
                    .label("Add Catagories")
                    .icon_name("plus-large-symbolic")
                    .margin_start(5)
                    .margin_end(5)
                    .build();
                let button = gtk::Button::builder()
                    .css_classes(["circular", "suggested-action"])
                    .child(&button_content)
                    .build();

                imp.categories_box.append(&button)
            }
        );
        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = icon)]
            self.imp().app_icon,
            #[weak(rename_to = stack)]
            self.imp().icon_stack,
            async move {
                while let Ok(res) = receiver.recv().await {
                    stack.set_visible_child_name("present");
                    icon.set_from_file(Some(res));
                }
            }
        ));
    }

    pub fn setup_binds(&self) {
        let imp = self.imp();

        self.bind_property("visible-editor", &imp.stack.get(), "visible-child-name")
            .bidirectional()
            .sync_create()
            .build();
    }
}
