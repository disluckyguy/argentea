use adw::{glib, gdk};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::cell::{Cell, RefCell};
use glib::Properties;
use crate::runtime;

#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::ArgenteaScreenshotTile)]
#[template(file = "src/ui/screenshot-tile.blp")]
pub struct ArgenteaScreenshotTile {
    #[property(get, set = Self::set_uri, construct)]
    pub uri: RefCell<String>,
    #[template_child]
    pub picture_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub picture: TemplateChild<gtk::Picture>,
}

#[glib::object_subclass]
impl ObjectSubclass for ArgenteaScreenshotTile {
    const NAME: &'static str = "ArgenteaScreenshotTile";
    type Type = super::ArgenteaScreenshotTile;
    type ParentType = gtk::FlowBoxChild;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for ArgenteaScreenshotTile {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArgenteaScreenshotTile {}

impl FlowBoxChildImpl for ArgenteaScreenshotTile {}

impl ArgenteaScreenshotTile {
    pub fn set_uri(&self, value: String)  {
        let picture = self.picture.get();
        let (sender, receiver) = async_channel::bounded(1);
        self.picture_stack.set_visible_child_name("spinner");
        runtime().spawn(glib::clone!(
            #[strong]
            sender,
            #[strong]
            value,
            async move {
                let response = reqwest::get(value)
                    .await
                    .expect("failed to get image")
                    .bytes()
                    .await
                    .expect("failed to convert to bytes");

                let bytes: Vec<u8> = response.to_vec();

                let gbytes = glib::Bytes::from(bytes.as_slice());
                sender.send(gbytes).await.expect("thread must be open");
            }
        ));
        glib::spawn_future_local(glib::clone!(
            #[weak]
            picture,
            #[weak(rename_to = stack)]
            self.picture_stack,
            async move {
                while let Ok(data) = receiver.recv().await {
                    let texture = gdk::Texture::from_bytes(&data).expect("failed to make texture");
                    picture.set_paintable(Some(&texture));
                    stack.set_visible_child_name("picture");
                }
            }
        ));
        *self.uri.borrow_mut() = value;
    }
}
