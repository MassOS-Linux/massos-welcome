use gettextrs::gettext;
use glib::clone;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use libhandy::prelude::{CarouselExt, CarouselIndicatorDotsExt, HeaderBarExt};

pub struct PaginatorWidget {
    pub widget: gtk::Box,
    carousel: libhandy::Carousel,
    carousel_dots: libhandy::CarouselIndicatorDots,
    headerbar: libhandy::HeaderBar,
    pages: RefCell<Vec<gtk::Widget>>,
    current_page: RefCell<u32>,
    next_btn: gtk::Button,
    start_btn: gtk::Button,
    finish_btn: gtk::Button,
    close_btn: gtk::Button,
    previous_btn: gtk::Button,
}

impl PaginatorWidget {
    pub fn new() -> Rc<Self> {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let paginator = Rc::new(Self {
            widget,
            carousel: libhandy::Carousel::new(),
            carousel_dots: libhandy::CarouselIndicatorDots::new(),
            headerbar: libhandy::HeaderBar::new(),
            start_btn: gtk::Button::with_label(&gettext("_Start")),
            next_btn: gtk::Button::with_label(&gettext("_Next")),
            finish_btn: gtk::Button::with_label(&gettext("_Done")),
            close_btn: gtk::Button::with_label(&gettext("_Previous")),
            previous_btn: gtk::Button::with_label(&gettext("_Previous")),
            pages: RefCell::new(Vec::new()),
            current_page: RefCell::new(0),
        });
        paginator.init(paginator.clone());
        paginator
    }

    pub fn try_next(&self) -> Option<()> {
        let p = *self.current_page.borrow() + 1;
        if p == self.carousel.get_n_pages() {
            return None;
        }
        self.set_page(p);
        Some(())
    }

    pub fn try_previous(&self) -> Option<()> {
        let p = *self.current_page.borrow();
        if p == 0 {
            return None;
        }
        self.set_page(p - 1);
        Some(())
    }

    pub fn add_page(&self, page: gtk::Widget) {
        let page_nr = self.pages.borrow().len();
        self.carousel.insert(&page, page_nr as i32);
        self.pages.borrow_mut().push(page);

        self.update_position();
    }

    fn update_position(&self) {
        let position = self.carousel.get_position();
        let page_nr = position.round() as u32;

        let n_pages = self.carousel.get_n_pages() as f64;
        let forelast_page = n_pages - 2.0;
        let last_page = n_pages - 1.0;

        let (opacity_finish, opacity_previous, opacity_start, opacity_next) =
            if (0.0..1.0).contains(&position) {
                (0.0, position, 1.0, position)
            } else if (0.0 <= position) && (position <= forelast_page) {
                (0.0, 1.0, 1f64 - position, 1.0)
            } else if (forelast_page < position) && (position <= last_page) {
                (position - forelast_page, 1.0, 0.0, 1.0)
            } else {
                panic!("Position of the carousel is outside the allowed range");
            };

        self.start_btn.set_opacity(opacity_start);
        self.start_btn.set_visible(opacity_start > 0_f64);

        self.next_btn.set_opacity(opacity_next);
        self.next_btn.set_visible(opacity_next > 0_f64);

        self.finish_btn.set_opacity(opacity_finish);
        self.finish_btn.set_visible(opacity_finish > 0_f64);

        self.previous_btn.set_opacity(opacity_previous);
        self.previous_btn.set_visible(opacity_previous > 0_f64);

        self.current_page.replace(page_nr);
    }

    fn init(&self, p: Rc<Self>) {
        self.carousel_dots.show();
        self.carousel_dots.set_carousel(Some(&self.carousel));
        self.carousel.set_property_expand(true);
        self.carousel.set_animation_duration(300);
        self.carousel.show();

        self.carousel
            .connect_property_position_notify(clone!(@weak p => move |_| {
                p.update_position();
            }));
        self.start_btn
            .get_style_context()
            .add_class("suggested-action");
        self.start_btn.set_use_underline(true);
        self.start_btn.set_action_name(Some("app.start-tour"));
        self.start_btn.show();

        self.next_btn
            .get_style_context()
            .add_class("suggested-action");
        self.next_btn.set_use_underline(true);
        self.next_btn.set_action_name(Some("app.next-page"));

        self.close_btn.set_use_underline(true);
        self.close_btn.set_action_name(Some("app.quit"));
        self.close_btn.show();

        self.finish_btn
            .get_style_context()
            .add_class("suggested-action");
        self.finish_btn.set_use_underline(true);
        self.finish_btn.set_action_name(Some("app.quit"));

        self.previous_btn.set_use_underline(true);
        self.previous_btn.set_action_name(Some("app.previous-page"));

        let previous_overlay = gtk::Overlay::new();
        previous_overlay.add(&self.close_btn);
        previous_overlay.add_overlay(&self.previous_btn);
        previous_overlay.show();

        let next_overlay = gtk::Overlay::new();
        next_overlay.add(&self.next_btn);
        next_overlay.add_overlay(&self.finish_btn);
        next_overlay.show();

        let start_overlay = gtk::Overlay::new();
        start_overlay.add(&self.start_btn);
        start_overlay.add_overlay(&next_overlay);
        start_overlay.set_overlay_pass_through(&next_overlay, true);
        start_overlay.show();

        let btn_size_group = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);
        btn_size_group.add_widget(&self.previous_btn);
        btn_size_group.add_widget(&self.close_btn);
        btn_size_group.add_widget(&next_overlay);
        btn_size_group.add_widget(&start_overlay);
        btn_size_group.add_widget(&self.finish_btn);

        self.headerbar.set_custom_title(Some(&self.carousel_dots));
        self.headerbar.pack_start(&previous_overlay);
        self.headerbar.pack_end(&start_overlay);
        self.headerbar.set_show_close_button(false);
        self.headerbar.show();

        self.widget.add(&self.headerbar);
        self.widget.add(&self.carousel);
        self.widget.show();
    }

    pub fn set_page(&self, page_nr: u32) {
        if page_nr < self.carousel.get_n_pages() {
            let pages = &self.pages.borrow();
            let page = pages.get(page_nr as usize).unwrap();
            self.carousel.scroll_to(page);
        }
    }
}
