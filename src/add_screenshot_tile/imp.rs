use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;

#[derive(CompositeTemplate, Default)]
#[template(file = "src/ui/add-screenshot-tile.blp")]
pub struct ArgenteaAddScreenshotTile {
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaAddScreenshotTile {
    const NAME: &'static str = "ArgenteaAddScreenshotTile";
    type Type = super::ArgenteaAddScreenshotTile;
    type ParentType = gtk::FlowBoxChild;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for ArgenteaAddScreenshotTile {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArgenteaAddScreenshotTile {}

impl FlowBoxChildImpl for ArgenteaAddScreenshotTile {}
