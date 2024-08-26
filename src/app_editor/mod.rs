mod imp;
use crate::add_screenshot_tile::ArgenteaAddScreenshotTile;
use crate::app_version_row::ArgenteaAppVersionRow;
use crate::screenshot_tile::ArgenteaScreenshotTile;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gdk, gio, glib};
use glib::Object;
use libappstream::prelude::*;
use libappstream::ColorKind;
use libappstream::ColorSchemeKind;
use libappstream::ControlKind;
use libappstream::Metadata;
use libappstream::ReleaseKind;
use libappstream::UrlKind;
use sourceview5::prelude::*;
use xmltree::Element;

glib::wrapper! {
    pub struct ArgenteaAppEditor(ObjectSubclass<imp::ArgenteaAppEditor>)
    @extends adw::BreakpointBin, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaAppEditor {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn metadata(&self) -> libappstream::Metadata {
        self.imp()
            .metadata
            .borrow()
            .clone()
            .expect("failed to get componenent")
    }

    pub fn setup_source_view(&self) {
        let style_manager = adw::StyleManager::default();
        let text_buffer = self.imp().source_view.buffer();
        let source_buffer = text_buffer
            .downcast_ref::<sourceview5::Buffer>()
            .expect("buffer must be SourceBuffer");
        let language_manager = sourceview5::LanguageManager::default();
        let language = language_manager
            .language("xml")
            .expect("language not present");
        source_buffer.set_language(Some(&language));
        style_manager
            .bind_property("dark", source_buffer, "style-scheme")
            .transform_to(move |_, is_dark: bool| {
                let source_style_manager = sourceview5::StyleSchemeManager::default();
                if is_dark {
                    return Some(
                        source_style_manager
                            .scheme("Adwaita-dark")
                            .expect("failed to get scheme"),
                    );
                }
                Some(
                    source_style_manager
                        .scheme("Adwaita")
                        .expect("failed to get scheme"),
                )
            })
            .sync_create()
            .build();
    }

    pub fn sync_metadata(&self, file: &gio::File) {
        let metadata = Metadata::new();

        let appstream_file =
            libappstream::gio::File::for_path(file.path().expect("path doesn't exist"));

        metadata
            .parse_file(&appstream_file, libappstream::FormatKind::Unknown)
            .unwrap();

        self.imp().metadata.replace(Some(metadata));

        let imp = self.imp();

        let (sender, receiver) = async_channel::bounded(1);

        let metadata = self.metadata();

        let component = metadata.component().expect("failed to get component");

        let text = metadata
            .component_to_metainfo(libappstream::FormatKind::Xml)
            .map(|s| s.to_string())
            .unwrap_or("".to_string());
        let tree = Element::parse(text.as_bytes()).expect("failed to parse to Element");

        imp.source_view.buffer().set_text(&text);

        imp.app_name.set_text(
            &component
                .name()
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );
        if let Some(developer) = tree.get_child("developer") {
            let name = developer
                .get_child("name")
                .map_or(None, |v| {
                    Some(v.get_text().map_or(String::new(), |v| v.to_string()))
                })
                .unwrap_or("".to_string());
            imp.developer_name.set_text(&name);
        }

        imp.mouse_support.set_active(false);
        imp.keyboard_support.set_active(false);
        imp.gamepad_support.set_active(false);
        imp.touchscreen_support.set_active(false);
        imp.graphics_tablet_support.set_active(false);

        for control in &component.supports() {
            match control.value_control_kind() {
                ControlKind::Pointing => imp.mouse_support.set_active(true),
                ControlKind::Keyboard => imp.keyboard_support.set_active(true),
                ControlKind::Gamepad => imp.gamepad_support.set_active(true),
                ControlKind::Touch => imp.touchscreen_support.set_active(true),
                ControlKind::Tablet => imp.graphics_tablet_support.set_active(true),
                _ => {}
            }
        }

        imp.app_summary.set_text(
            &component
                .summary()
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );
        imp.contact_email.set_text(
            &component
                .url(UrlKind::Contact)
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );
        imp.app_homepage.set_text(
            &component
                .url(UrlKind::Homepage)
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );
        imp.app_donations.set_text(
            &component
                .url(UrlKind::Donation)
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );
        imp.app_translations.set_text(
            &component
                .url(UrlKind::Translate)
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );
        imp.app_bug_tracker.set_text(
            &component
                .url(UrlKind::Bugtracker)
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );

        imp.metadata_license.set_selected(0);
        if let Some(metadata_license) = component.metadata_license() {
            let model = imp.metadata_license.model().expect("failed to get model");
            let string_list = model
                .downcast_ref::<gtk::StringList>()
                .expect("model must be string list");
            for i in 0..string_list.n_items() {
                if string_list.string(i).expect("failed to get string") == metadata_license.as_str()
                {
                    imp.metadata_license.set_selected(i);
                }
            }
        }
        imp.project_license.set_selected(0);
        if let Some(project_license) = component.project_license() {
            let model = imp.project_license.model().expect("failed to get model");
            let string_list = model
                .downcast_ref::<gtk::StringList>()
                .expect("model must be string list");
            for i in 0..string_list.n_items() {
                if string_list.string(i).expect("failed to get string") == project_license.as_str()
                {
                    imp.project_license.set_selected(i);
                }
            }
        }

        let sender = sender.clone();
        let path = file.path().expect("failed to get path");
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
            }
        ));

        imp.description_text.buffer().set_text(
            &component
                .description()
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        );

        imp.screenshot_box.remove_all();

        for screenshot in &component.screenshots_all() {
            let source_images: Vec<String> = screenshot
                .images()
                .clone()
                .into_iter()
                .filter(|i| i.kind() == libappstream::ImageKind::Source)
                .filter_map(|i| i.url().map(|url| url.to_string()))
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

        for requirement in component.requires() {
            if requirement.value_int() != 0 {
                imp.min_length.set_value(requirement.value_int() as f64);
                imp.use_display_length.set_active(true);
                break;
            }
        }

        imp.app_color_label.set_label("");
        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            "
                    .app-color-button {
                        background-color: #ffffff;
                    }
                ",
        );

        let display = gdk::Display::default().expect("failed to get display");
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        imp.app_color_dark_label.set_label("");
        let provider = gtk::CssProvider::new();
        provider.load_from_string(
            "
                    .app-color-button-dark {
                        background-color: #000000;
                    }
                ",
        );

        let display = gdk::Display::default().expect("failed to get display");
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        if let Some(branding) = component.branding() {
            let color = branding
                .color(ColorKind::Primary, ColorSchemeKind::Dark)
                .map(|s| s.to_string())
                .unwrap_or("".to_string());

            imp.app_color_dark_label.set_label(&color);
            let provider = gtk::CssProvider::new();
            provider.load_from_string(&format!(
                "
                                        .app-color-button-dark {{
                                            background-color: {};
                                        }}
                                    ",
                color
            ));

            let display = gdk::Display::default().expect("failed to get display");
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let color = branding
                .color(ColorKind::Primary, ColorSchemeKind::Light)
                .map(|s| s.to_string())
                .unwrap_or("".to_string());

            imp.app_color_label.set_label(&color);
            let provider = gtk::CssProvider::new();
            provider.load_from_string(&format!(
                "
                    .app-color-button {{
                        background-color: {};
                    }}
                ",
                color
            ));

            let display = gdk::Display::default().expect("failed to get display");
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        imp.version_history_list.remove_all();
        let releases = component
            .releases_plain()
            .map(|l| l.entries())
            .unwrap_or(Vec::new());
        for release in &releases {
            let row = ArgenteaAppVersionRow::new(
                &release
                    .version()
                    .map(|s| s.to_string())
                    .unwrap_or("".to_string()),
                &release
                    .date()
                    .map(|s| s.to_string())
                    .unwrap_or("".to_string()),
                release.kind() == ReleaseKind::Development,
            );
            imp.version_history_list.append(&row);
        }

        let button_row = adw::ButtonRow::builder()
            .start_icon_name("plus-large-symbolic")
            .build();
        imp.version_history_list.append(&button_row);

        imp.categories_box.remove_all();

        let categories_text = component.categories();

        for catagory in categories_text {
            let label = gtk::Label::builder()
                .label(&catagory.to_string())
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

            let child = gtk::FlowBoxChild::builder().child(&content_box).build();

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

        imp.categories_box.append(&button);

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

    pub fn setup_callbacks(&self) {
        let _imp = self.imp();
    }

    pub fn setup_binds(&self) {
        let imp = self.imp();

        self.bind_property("visible-editor", &imp.stack.get(), "visible-child-name")
            .bidirectional()
            .sync_create()
            .build();
    }
}
