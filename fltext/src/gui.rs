use {
    crate::{cbs, dialogs, fbr, utils},
    fltk::{enums::*, prelude::*, window::Window,group::Flex, *},
    fltk_theme::{color_themes, ColorTheme},
    std::path::{Path, PathBuf},
};

#[cfg(feature = "term")]
use fltk_term as term;

#[cfg(feature = "highlight")]
use crate::highlight;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const MENU_HEIGHT: i32 = if cfg!(target_os = "macos") { 1 } else { 30 };

pub fn init_gui(current_file: &Option<PathBuf>, current_path: &Path) -> app::App {
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::set_font(Font::Courier);
    app::set_menu_linespacing(10);
    let mut buf = text::TextBuffer::default();
    buf.set_tab_distance(4);

    let _find_dialog = dialogs::FindDialog::new();
    let _replace_dialog = dialogs::ReplaceDialog::new();
    let _image_dialog = dialogs::ImageDialog::new();

    let mut popup = menu::MenuButton::default().with_type(menu::MenuButtonType::Popup3);
    init_edit_menu(&mut popup, "");

    let mut window = window();

    let mut page = Flex::default_fill().column();
    let mut m = menu::SysMenuBar::default().with_id("menu");
    init_menu(&mut m, current_file.is_none());
    let mut row = group::Flex::default();
    row.set_pad(0);
    let fbr = fbr::Fbr::new(current_path);
    if current_file.is_none() {
        row.fixed(&*fbr, 180);
    } else {
        row.fixed(&*fbr, 1);
    }
    let mut fbr_splitter = frame::Frame::default();
    fbr_splitter.handle(cbs::fbr_splitter_cb);
    row.fixed(&fbr_splitter, 4);
    let mut col = group::Flex::default().column();
    col.set_pad(0);
    let mut tabs = group::Tabs::default().with_id("tabs");
    tabs.handle(move |t, ev| tabs_handle(t, ev, &mut popup));
    tabs.handle_overflow(group::TabsOverflow::Pulldown);
    tabs.end();
    tabs.auto_layout();
    #[cfg(feature = "term")]
    {
        let mut tab_splitter = frame::Frame::default();
        tab_splitter.handle(cbs::tab_splitter_cb);
        col.fixed(&tab_splitter, 4);
        let term = term::PPTerm::default();
        col.fixed(&*term, 160);
    }
    col.end();
    row.end();
    let info = frame::Frame::default()
        .with_label(&format!(
            "Directory: {}",
            utils::strip_unc_path(current_path)
        ))
        .with_align(enums::Align::Left | enums::Align::Inside)
        .with_id("info");
    page.set_pad(2);
    page.fixed(&m, MENU_HEIGHT);
    page.fixed(&info, 20);
    page.end();
    //w.resizable(&row);
    window.end();
    window.show();
    app::App::default()
}

fn window() -> Window {
    const NAME: &str = "FlText";
    let mut element = Window::default().with_size(WIDTH, HEIGHT).with_label(NAME);
    element.set_xclass(NAME);
    element.make_resizable(true);
    element.set_callback(cbs::win_cb);
    element
}

pub fn tabs_handle(t: &mut group::Tabs, ev: Event, popup: &mut menu::MenuButton) -> bool {
    match ev {
        Event::Push => {
            if app::event_mouse_button() == app::MouseButton::Right
                && app::event_y() > t.y() + 30
                && t.children() > 0
            {
                popup.popup();
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn init_edit_menu(m: &mut (impl MenuExt + 'static), header: &str) {
    m.add(
        &format!("{}@#undo  &Undo\t", header),
        Shortcut::Ctrl | 'z',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        &format!("{}@#redo  &Redo\t", header),
        Shortcut::Ctrl | 'y',
        menu::MenuFlag::MenuDivider,
        cbs::menu_cb,
    );
    m.add(
        &format!("{}Cut\t", header),
        Shortcut::Ctrl | 'x',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        &format!("{}Copy\t", header),
        Shortcut::Ctrl | 'c',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        &format!("{}Paste\t", header),
        Shortcut::Ctrl | 'v',
        menu::MenuFlag::MenuDivider,
        cbs::menu_cb,
    );
    m.add(
        &format!("{}@#search  &Find\t", header),
        Shortcut::Ctrl | 'f',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        &format!("{}Replace\t", header),
        Shortcut::Ctrl | 'h',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
}
pub fn init_menu(m: &mut (impl MenuExt + 'static), load_dir: bool) {
    m.add(
        "&File/@#filenew  &New File...\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        "&File/@#fileopen  New Dir...\t",
        Shortcut::Ctrl | Shortcut::Shift | 'n',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        "&File/@#fileopen  &Open...\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        "&File/@#filesave  &Save\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        "&File/@#filesaveas  Save as...\t",
        Shortcut::Ctrl | Shortcut::Shift | 'w',
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
    m.add(
        "&File/@#filesaveas  Save All\t",
        Shortcut::None,
        menu::MenuFlag::MenuDivider,
        cbs::menu_cb,
    );
    let idx = m.add(
        "&File/@#1+  Quit\t",
        Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        move |_| {
            app::handle_main(Event::Close).unwrap();
        },
    );
    m.at(idx)
        .unwrap()
        .set_label_color(Color::from_hex(0xdc322f));
    init_edit_menu(m, "&Edit/");
    let idx = m.add(
        "&View/File browser\t",
        Shortcut::None,
        menu::MenuFlag::Toggle,
        cbs::menu_cb,
    );
    if load_dir {
        m.at(idx).unwrap().set();
    }
    #[cfg(feature = "term")]
    {
        let idx = m.add(
            "&View/Terminal\t",
            Shortcut::None,
            menu::MenuFlag::Toggle,
            cbs::menu_cb,
        );
        m.at(idx).unwrap().set();
    }
    m.add(
        "&Help/About\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        cbs::menu_cb,
    );
}

pub fn build_editor(id: &str) -> text::TextEditor {
    let mut texteditor = text::TextEditor::default().with_id(id);
    texteditor.set_color(Color::from_hex(0x002b36));
    texteditor.set_linenumber_width(40);
    texteditor.set_linenumber_size(12);
    texteditor.set_linenumber_fgcolor(Color::from_hex(0xb58900));
    texteditor.set_linenumber_bgcolor(Color::Background.darker());
    texteditor.set_text_font(Font::Courier);
    texteditor.set_trigger(CallbackTrigger::Changed);
    texteditor.set_callback(cbs::editor_cb);
    texteditor
}

pub fn create_ed(
    tabs: &mut group::Tabs,
    id: &str,
    current_path: &Option<PathBuf>,
) -> text::TextEditor {
    tabs.begin();
    let mut edrow = group::Flex::default()
        .row()
        .with_label(if let Some(current_path) = current_path.as_ref() {
            if current_path.is_dir() {
                "untitled"
            } else {
                current_path.file_name().unwrap().to_str().unwrap()
            }
        } else {
            "untitled"
        })
        .with_id(id);
    edrow.set_trigger(CallbackTrigger::Closed);
    edrow.set_callback(cbs::tab_close_cb);
    let mut ed = build_editor("ed");
    edrow.end();
    tabs.end();
    tabs.auto_layout();
    tabs.set_value(&edrow).ok();

    let mut buf = text::TextBuffer::default();
    buf.set_tab_distance(4);
    if let Some(p) = current_path.as_ref() {
        buf.load_file(p).ok();
        #[cfg(feature = "highlight")]
        highlight::highlight(p, &mut ed, &mut buf);
    }
    ed.set_buffer(buf);
    ed
}
