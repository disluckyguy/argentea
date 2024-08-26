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

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib, gdk};
use crate::app_editor::ArgenteaAppEditor;
use crate::app_preview::ArgenteaAppPreview;
use libappstream::Metadata;
use libappstream::prelude::*;
use xmltree::Element;


mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/ui/window.blp")]
    pub struct ArgenteaWindow {
        // Template widgets
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub bin: TemplateChild<adw::Bin>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub editor_toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub app_editor: TemplateChild<ArgenteaAppEditor>,
        #[template_child]
        pub app_preview: TemplateChild<ArgenteaAppPreview>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ArgenteaWindow {
        const NAME: &'static str = "ArgenteaWindow";
        type Type = super::ArgenteaWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ArgenteaWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_binds();
            obj.setup_drop_target();
        }
    }
    impl WidgetImpl for ArgenteaWindow {}
    impl WindowImpl for ArgenteaWindow {}
    impl ApplicationWindowImpl for ArgenteaWindow {}
    impl AdwApplicationWindowImpl for ArgenteaWindow {}

    #[gtk::template_callbacks]
    impl ArgenteaWindow {
        #[template_callback]
        fn open_file_dialog(&self) {
            // let filter = gtk::FileFilter::new();
            // filter.add_suffix("metainfo.xml");
            // filter.add_suffix("metainfo.xml.in");
            // let filters = gio::ListStore::from_iter(vec!(filter).into_iter());

            let file_dialog = gtk::FileDialog::builder()
                .accept_label("open")
                .title("Open Metainfo File")
                // .filters(&filters)
                .build();


            file_dialog.open(Some(self.obj().as_ref()), None::<&gio::Cancellable>,glib::clone!(
            #[weak(rename_to = obj)]
            self.obj(),
            move |result| {
                if let Ok(file) = result {
                    let file_name = file.parse_name().to_string();

                    if file_name.len() < 12 {
                        let toast = adw::Toast::new("Invalid file name");
                        obj.imp().toast_overlay.add_toast(toast);
                    }

                    let metadata = Metadata::new();

                    let appstream_file = libappstream::gio::File::for_path(file.path().expect("path doesn't exist"));
                    match metadata.parse_file(&appstream_file, libappstream::FormatKind::Unknown) {
                        Ok(()) => {
                            obj.imp().app_editor.imp().metadata.replace(Some(metadata.clone()));
                            obj.imp().stack.set_visible_child_name("preview-page");
                        }
                        Err(err) => {
                            let toast = adw::Toast::new(&err.to_string());

                            obj.imp().toast_overlay.add_toast(toast);
                        }
                    }
                }


            }));
        }

    }
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

    pub fn setup_drop_target(&self) {
        let drop_target = gtk::DropTarget::new(gio::File::static_type(), gdk::DragAction::COPY | gdk::DragAction::MOVE);


        drop_target.connect_drop(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[upgrade_or_panic]
            move |_,value, _, _| {
                if let Ok(file) = value.get::<gio::File>() {
                    let file_name = file.parse_name().to_string();

                    if file_name.len() < 12 {
                        let toast = adw::Toast::new("Invalid file name");
                        obj.imp().toast_overlay.add_toast(toast);
                    }

                    let metadata = Metadata::new();

                    let appstream_file = libappstream::gio::File::for_path(file.path().expect("path doesn't exist"));
                    match metadata.parse_file(&appstream_file, libappstream::FormatKind::Unknown) {
                        Ok(()) => {
                            obj.imp().app_editor.sync_metadata(&file);
                            obj.imp().stack.set_visible_child_name("preview-page");
                        }
                        Err(err) => {
                            let toast = adw::Toast::new(&err.to_string());

                            obj.imp().toast_overlay.add_toast(toast);
                        }
                    }
                }
                false
            }
        ));

        drop_target.connect_enter(glib::clone!(
            #[weak(rename_to = bin)]
            self.imp().bin,
            #[upgrade_or_panic]
            move |_,_, _| {
                bin.add_css_class("overlay-drag-area-on-enter");
                gdk::DragAction::COPY
            }
        ));

        drop_target.connect_leave(glib::clone!(
            #[weak(rename_to = bin)]
            self.imp().bin,
            #[upgrade_or_panic]
            move |_| {
                bin.remove_css_class("overlay-drag-area-on-enter");
            }
        ));

        self.add_controller(drop_target);
    }

    pub fn setup_binds(&self) {
        let imp = self.imp();

        imp.editor_toggle.bind_property("active", &imp.app_editor.get(), "visible-editor")
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


