mod imp;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;

glib::wrapper! {
    pub struct ArgenteaLozenge(ObjectSubclass<imp::ArgenteaLozenge>)
    @extends gtk::Box, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ArgenteaLozenge {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setup_binds(&self) {
        let imp = self.imp();
        self.bind_property("icon-name", &imp.image.get(), "icon-name")
            .build();

        self.bind_property("icon-name", &imp.image.get(), "visible")
            .transform_to(move |_, icon_name: Option<String>| {
                if let Some(_) = icon_name {
                    Some(true)
                } else {
                    Some(false)
                }
            })
            .sync_create()
            .build();

        imp.image.bind_property("visible", &imp.label.get(), "visible")
            .invert_boolean()
            .sync_create()
            .bidirectional()
            .build();

        self.bind_property("text", &imp.label.get(), "label")
            .build();

        // self.bind_property("markup", &imp.label.get(), "markup")
        //     .build();


    }
}
