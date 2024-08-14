/* window.rs
 *
 * Copyright 2024 Mostafa
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use sourceview5::prelude::*;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use crate::app_context_bar::ArgenteaAppContextBar;
use crate::description_box::ArgenteaDescriptionBox;
use crate::app_version_history_row::ArgenteaAppVersionHistoryRow;
use crate::license_tile::ArgenteaLicenseTile;
use sourceview5::View;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/ui/window.blp")]
    pub struct ArgenteaWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub editor_toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub editor_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub context_bar: TemplateChild<ArgenteaAppContextBar>,
        #[template_child]
        pub app_description_box: TemplateChild<ArgenteaDescriptionBox>,
        #[template_child]
        pub version_history_row: TemplateChild<ArgenteaAppVersionHistoryRow>,
        #[template_child]
        pub license_tile: TemplateChild<ArgenteaLicenseTile>,
        #[template_child]
        pub source_view: TemplateChild<View>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ArgenteaWindow {
        const NAME: &'static str = "ArgenteaWindow";
        type Type = super::ArgenteaWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ArgenteaWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_source_view();
            obj.setup_binds();
        }
    }
    impl WidgetImpl for ArgenteaWindow {}
    impl WindowImpl for ArgenteaWindow {}
    impl ApplicationWindowImpl for ArgenteaWindow {}
    impl AdwApplicationWindowImpl for ArgenteaWindow {}
}

glib::wrapper! {
    pub struct ArgenteaWindow(ObjectSubclass<imp::ArgenteaWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl ArgenteaWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
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

    pub fn setup_binds(&self) {
        let imp = self.imp();

        imp.editor_toggle.bind_property("active", &imp.editor_stack.get(), "visible-child-name")
            .transform_to(move |_, is_active: bool| {
                if is_active {
                    return Some("source-view");
                }
                Some("gui-editor")
            })
            .sync_create()
            .build();
    }
}
