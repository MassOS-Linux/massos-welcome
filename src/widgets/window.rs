use crate::utils::i18n_f;
use gettextrs::gettext;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::pages::{ImagePageWidget, WelcomePageWidget};
use super::paginator::PaginatorWidget;
use crate::config::{APP_ID, PROFILE};

pub struct Window {
    pub widget: libhandy::ApplicationWindow,
    pub paginator: RefCell<Rc<PaginatorWidget>>,
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        let widget = libhandy::ApplicationWindow::new();
        widget.set_application(Some(app));

        let paginator = RefCell::new(PaginatorWidget::new());

        let mut window_widget = Window { widget, paginator };

        window_widget.init();
        window_widget
    }

    pub fn start_tour(&self) {
        self.paginator.borrow_mut().set_page(1);
    }

    pub fn reset_tour(&self) {
        self.paginator.borrow_mut().set_page(0);
    }

    fn init(&mut self) {
        self.widget.set_default_size(960, 720);
        self.widget.set_icon_name(Some(APP_ID));

        // Devel Profile
        if PROFILE == "Devel" {
            self.widget.get_style_context().add_class("devel");
        }
        self.paginator
            .borrow_mut()
            .add_page(WelcomePageWidget::new().widget.upcast::<gtk::Widget>());
        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/overview.svg",
                gettext("See Your Apps"),
                gettext("Your apps are neatly categorised. You can also search for apps."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/search.svg",
                gettext("Just Type to Search"),
                gettext("Type in the menu to search for apps, settings and more."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/workspaces.svg",
                gettext("Get New Apps"),
                gettext("With 'Software' you can find and install new apps."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/blank.svg",
                gettext("Customize Your Desktop"),
                gettext("You can use 'Settings Manager' to tweak options such as appearance."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        let name = glib::get_os_info("NAME").unwrap_or_else(|| "GNOME".into());
        let version = glib::get_os_info("VERSION").unwrap_or_else(|| "".into());
        let last_page = ImagePageWidget::new(
            "/org/gnome/Tour/ready-to-go.svg",
            gettext("That's it. You're ready!"),
            gettext("We hope your experience with MassOS will be an exceptional one!"),
        );
        last_page.widget.get_style_context().add_class("last-page");
        self.paginator
            .borrow_mut()
            .add_page(last_page.widget.upcast::<gtk::Widget>());

        self.widget.add(&self.paginator.borrow().widget);
    }
}
