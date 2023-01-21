use std::cell::RefCell;
use std::rc::Rc;

use gtk::gio::{self, Menu, SimpleAction};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, FileChooserAction, FileChooserNative, ResponseType,
    ScrolledWindow, TextBuffer, TextView, WrapMode,
};
use leptos_reactive::*;

#[derive(Copy, Clone)]
struct ValueSetter(WriteSignal<String>);

struct AppData {
    parent_window: Option<ApplicationWindow>,
}

fn main() {
    _ = create_scope(RuntimeId::default(), |cx| {
        let app_data = Rc::new(RefCell::new(AppData {
            parent_window: None,
        }));

        let app = Application::builder()
            .application_id("com.rust.gtk")
            .build();

        app.connect_activate(clone!(@weak app_data => move |app| build_ui(cx, app, app_data)));
        app.connect_startup(clone!(@weak app_data => move |app| build_menu(cx, app, app_data)));

        app.run();
    })
}

fn build_edit_box(cx: Scope) -> ScrolledWindow {
    let (text, set_text) = create_signal(cx, String::new());
    provide_context(cx, ValueSetter(set_text));

    let buf = TextBuffer::new(None);
    let scroll = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(
            &TextView::builder()
                .editable(true)
                .buffer(&buf)
                .wrap_mode(WrapMode::Word)
                .build(),
        )
        .build();

    create_effect(
        cx,
        clone!(@weak buf => move |_| {
            buf.set_text(text().as_ref());
        }),
    );

    scroll
}

fn build_ui(cx: Scope, app: &Application, app_data: Rc<RefCell<AppData>>) {
    let w = ApplicationWindow::builder()
        .application(app)
        .title("Hello, GTK!")
        .child(&build_edit_box(cx))
        .show_menubar(true)
        .build();

    w.present();
    app_data.borrow_mut().parent_window = Some(w);
}

fn build_menu(cx: Scope, app: &Application, app_data: Rc<RefCell<AppData>>) {
    let menubar = Menu::new();
    let file_menu = Menu::new();

    menubar.append_item(&{
        let file_op_item = gio::MenuItem::new(Some("File"), None);
        file_op_item.set_submenu(Some(&file_menu));
        file_op_item
    });

    file_menu.append_section(None, &{
        let file_op_section = Menu::new();
        file_op_section.append(Some("Open"), Some("app.open"));
        file_op_section
    });

    file_menu.append_section(None, &{
        let file_quit_section = Menu::new();
        file_quit_section.append(Some("Quit"), Some("app.quit"));
        file_quit_section
    });

    app.add_action(&{
        let act_quit = SimpleAction::new("quit", None);
        act_quit.connect_activate(clone!(@weak app => move |_, _| {
            app.quit();
        }));
        act_quit
    });

    app.add_action(&{
        let act_open = SimpleAction::new("open", None);
        act_open.connect_activate(clone!(@weak app, @weak app_data => move |_, _| {
            let app_data = app_data.borrow();
            let parent_window = app_data.parent_window.as_ref().unwrap();

            let dialog = FileChooserNative::builder()
                .title("Open File")
                .transient_for(parent_window)
                .action(FileChooserAction::Open)
                .accept_label("_Open")
                .cancel_label("_Cancel")
                .build();

            dialog.connect_response(glib::clone!(@strong dialog => move |_, response| {
                if response != ResponseType::Accept {
                    dialog.destroy();
                    return;
                }

                let file = dialog.file().unwrap();
                let contents = std::fs::read_to_string(file.path().unwrap()).unwrap();
                let set_text = use_context::<ValueSetter>(cx).unwrap().0;
                set_text(contents);
            }));
            dialog.show();
        }));
        act_open
    });
    app.set_menubar(Some(&menubar));
}
