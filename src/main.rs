use std::collections::HashMap;

use gtk::glib::clone;
use gtk::prelude::*;
use relm4::{adw, SharedState};
use gio::{AppInfo, File, Settings};
use gio::prelude::{AppInfoExt, FileExt};
use relm4::factory::FactoryVecDeque;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use itertools::{Itertools, EitherOrBoth};
use gio;
use relm4::prelude::*;
use relm4::gtk::prelude::{ButtonExt, WidgetExt, BoxExt, GtkWindowExt, ToggleButtonExt, ApplicationExtManual, ApplicationExt};


const EXCLUDED_APPS: [&str; 4] = [
    "com.Nosterx.BrowDi",
    "com.Nosterx.BrowDiIced",
    "browdi.desktop",
    "browdi-iced.desktop",
];


static APP_STATE: SharedState<bool> = SharedState::new();


#[derive(Debug)]
struct BrowserButtonInit {
    hotkey: Option<char>,
    icon: gio::Icon,
    name: String,
    width: u16,
    height: u16,
    margin_top: u16,
    margin_bottom: u16,
    margin_start: u16,
    margin_end: u16,
}


#[derive(Debug)]
#[tracker::track]
struct BrowserButton {
    icon: gio::Icon,
    name: String,
    width: u16,
    height: u16,
    margin_top: u16,
    margin_bottom: u16,
    margin_start: u16,
    margin_end: u16,
    show_hotkey_help: bool,
    hotkey_help_label: gtk::Label,
}


#[derive(Debug)]
enum BrowserButtonOutputMessage {
    Pressed(DynamicIndex),
}


#[derive(Debug)]
enum BrowserButtonInputMessage {
    Update,
}


#[relm4::factory]
impl FactoryComponent for BrowserButton {
    type Init = BrowserButtonInit;
    type Input = BrowserButtonInputMessage;
    type Output = BrowserButtonOutputMessage;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            gtk::Overlay {
                gtk::Button {
                    set_margin_top: self.margin_top.into(),
                    set_margin_bottom: self.margin_bottom.into(),
                    set_margin_start: self.margin_start.into(),
                    set_margin_end: self.margin_end.into(),
                    set_width_request: self.width.into(),
                    set_height_request: self.height.into(),
                    set_tooltip_text: Some(self.name.clone()).as_deref(),

                    gtk::Image::from_gicon(&self.icon) {
                        set_pixel_size: self.height.into(),
                    },

                    connect_clicked[sender, index] => move |_| {
                        sender.output(BrowserButtonOutputMessage::Pressed(index.clone())).unwrap();
                    },
                },

                #[track({self.changed(BrowserButton::show_hotkey_help()) && self.show_hotkey_help})]
                add_overlay: &self.hotkey_help_label,

                #[track({self.changed(BrowserButton::show_hotkey_help()) && !self.show_hotkey_help})]
                remove_overlay: &self.hotkey_help_label,
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        APP_STATE.subscribe(sender.input_sender(), |_| BrowserButtonInputMessage::Update);
        let hotkey_help_label = gtk::Label::builder()
            .label(format!("{}", init.hotkey.map(|x| x.to_string()).as_deref().unwrap_or("")))
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .margin_top(init.margin_end.into())
            .margin_start(init.margin_end.into())
            .css_classes(vec!["background", "button-label"])
            .build();

        hotkey_help_label.inline_css(
            "padding: 8px;
             opacity: 0.8;
             background-color: #242424;
             color: #ffffff;
             border-radius: 4px;"
        );

        Self {
            icon: init.icon,
            name: init.name,
            width: init.width,
            height: init.height,
            margin_top: init.margin_top,
            margin_bottom: init.margin_bottom,
            margin_start: init.margin_start,
            margin_end: init.margin_end,
            show_hotkey_help: false,
            hotkey_help_label,
            tracker: 0,
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        self.reset();
        match msg {
            BrowserButtonInputMessage::Update => {
                self.set_show_hotkey_help(*APP_STATE.read());
            },
        }
    }
}


#[derive(Debug, Clone)]
enum AppInputMessage {
    BrowserButtonPressed(usize),
    DomainToggleToggled(bool),
    FilesOpenRequested(Vec<File>),
    CurrentFileChanged,
    KeyPressed(gtk::gdk::Key),
    Quit,
    ShowFullUrlToggleToggled(bool),
    MenuOpened,
}

struct BrowDiInit {
    padding: u16,
    spacing: u16,
    button_height: u16,
    button_width: u16,
    browsers: Vec<AppInfo>,
}


impl Default for BrowDiInit {
    fn default() -> Self {
        let browsers = AppInfo::recommended_for_type("x-scheme-handler/http")
            .into_iter()
            .filter(|app_info| app_info.id().is_some_and(|id| !EXCLUDED_APPS.contains(&id.as_str())))
            .collect();
        BrowDiInit {
            padding: 5,
            spacing: 5,
            button_height: 150,
            button_width: 150,
            browsers,
        }
    }

}


#[tracker::track]
struct BrowDiModel {
    margin: u16,
    spacing: u16,
    button_height: u16,
    button_width: u16,
    browsers: Vec<AppInfo>,
    default_for_domain: bool,
    files: Vec<File>,
    is_domain_toggle_visible: bool,
    current_uri: Option<String>,
    current_domain: Option<String>,
    settings: Settings,
    show_keyboard_shortcuts_tooltips: bool,
    show_full_url: bool,
    default_for_domain_toggle_label: gtk::Label,
    menu_label: gtk::Label,
    activate_menu: bool,
    #[do_not_track]
    buttons: FactoryVecDeque<BrowserButton>,
    hotkeys: Vec<char>,
}


#[relm4::component]
impl SimpleComponent for BrowDiModel {

    /// The type of the messages that this component can receive.
    type Input = AppInputMessage;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = BrowDiInit;

    view! {
        main_window = adw::Window {
            set_decorated: true,
            set_resizable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: model.spacing as i32,
                set_margin_start: model.margin as i32,
                set_margin_top: model.margin as i32,
                set_margin_bottom: model.margin as i32,
                set_valign: gtk::Align::Center,

                #[local]
                browser_buttons_vbox -> gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                },

                gtk::CenterBox {
                    set_margin_all: 0,

                    #[wrap(Some)]
                    set_start_widget: default_for_domain_toggle = &gtk::Overlay {
                        gtk::ToggleButton {
                            set_tooltip: "Remeber selection for the domain of the url",
                            set_margin_all: 0,

                            #[watch]
                            set_visible: model.is_domain_toggle_visible,

                            #[watch]
                            set_active: model.default_for_domain,

                            connect_toggled[sender] => move |btn| {
                                sender.input(AppInputMessage::DomainToggleToggled(btn.is_active()));
                            },
                        },

                        #[track({model.changed(BrowDiModel::show_keyboard_shortcuts_tooltips()) && model.show_keyboard_shortcuts_tooltips})]
                        add_overlay: &model.default_for_domain_toggle_label,

                        #[track({model.changed(BrowDiModel::show_keyboard_shortcuts_tooltips()) && !model.show_keyboard_shortcuts_tooltips})]
                        remove_overlay: &model.default_for_domain_toggle_label,
                    },

                    #[wrap(Some)]
                    set_center_widget = &gtk::Frame{
                        set_margin_start: model.margin as i32,
                        set_margin_end: model.margin as i32,

                        gtk::Label {
                            set_margin_top: 0,
                            set_margin_bottom: 0,
                            set_margin_start: model.margin as i32,
                            set_margin_end: model.margin as i32,
                            set_justify: gtk::Justification::Center,

                            #[watch]
                            set_text: &model.current_domain
                                        .clone()
                                        .filter(|_| !model.show_full_url)
                                        .unwrap_or(
                                            model.current_uri
                                                 .clone()
                                                 .unwrap_or(String::from(""))),

                            #[watch]
                            set_tooltip_text: model.current_uri.clone().as_deref(),
                        },
                    },

                    #[wrap(Some)]
                    set_end_widget = &gtk::Overlay {
                        gtk::MenuButton {
                            set_direction: gtk::ArrowType::None,
                            set_margin_top: 0,
                            set_margin_bottom: 0,
                            set_margin_start: model.margin as i32,
                            set_margin_end: model.margin as i32,

                            #[track(model.changed(BrowDiModel::activate_menu()) && model.activate_menu)]
                            activate: (),

                            #[wrap(Some)]
                            set_popover: popover = &gtk::Popover {
                                set_position: gtk::PositionType::Bottom,

                                connect_show => AppInputMessage::MenuOpened,

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_spacing: 5,
                                    gtk::Button {
                                        set_label: "Quit",
                                        connect_clicked => AppInputMessage::Quit
                                    },
                                    gtk::ToggleButton {
                                        set_label: "Show full url",

                                        #[watch]
                                        set_active: model.show_full_url,

                                        connect_toggled[sender] => move |btn| {
                                            sender.input(AppInputMessage::ShowFullUrlToggleToggled(btn.is_active()));
                                        },
                                    },
                                },
                            },
                        },
                        #[track({model.changed(BrowDiModel::show_keyboard_shortcuts_tooltips()) && model.show_keyboard_shortcuts_tooltips})]
                        add_overlay: &model.menu_label,

                        #[track({model.changed(BrowDiModel::show_keyboard_shortcuts_tooltips()) && !model.show_keyboard_shortcuts_tooltips})]
                        remove_overlay: &model.menu_label,
                    },
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
        let key_controller = gtk::EventControllerKey::new();
        key_controller.connect_key_pressed(clone!(@strong sender => move |_, keyval, _keycode, _state| {
            sender.input(AppInputMessage::KeyPressed(gtk::gdk::Key::from(keyval)));
            gio::glib::Propagation::Proceed
        }));
        window.add_controller(key_controller.clone());

        let mut browser_buttons =
            FactoryVecDeque::<BrowserButton>::builder()
                .launch_default()
                .forward(sender.input_sender(), |msg| match msg {
                    BrowserButtonOutputMessage::Pressed(index) => AppInputMessage::BrowserButtonPressed(index.current_index()),
                });

        let browser_buttons_vbox: gtk::Box = browser_buttons.widget().clone();
        let hotkeys = vec!['A', 'B', 'C', 'E', 'F', 'G', 'I', 'J', 'K', 'L', 'N', 'O', 'P', 'Q', 'R', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        for browser_and_hotkey in init.browsers.iter().zip_longest(hotkeys.iter()) {
            let hotkey: Option<char>;
            let browser: AppInfo;
            match browser_and_hotkey {
                EitherOrBoth::Right(_) => break,
                EitherOrBoth::Both(b, h) => {
                    hotkey = Some(h.to_owned());
                    browser = b.clone();
                },
                EitherOrBoth::Left(b) => {
                    hotkey = None;
                    browser = b.clone();
                },
            };
            browser_buttons.guard().push_back(
                BrowserButtonInit{
                    hotkey,
                    icon: browser.icon().unwrap(),
                    name: browser.name().to_string(),
                    width: init.button_width,
                    height: init.button_height,
                    margin_top: 0,
                    margin_bottom: 0,
                    margin_start: 0,
                    margin_end: init.spacing,
                }
            );
        }

        let settings = Settings::new("com.Nosterx.BrowDi");
        let menu_label = gtk::Label::builder().label("M").opacity(0.8).css_classes(vec!["background"]).build();
        let domain_label = gtk::Label::builder().label("D").opacity(0.8).css_classes(vec!["background"]).build();
        let model = BrowDiModel {
            margin: init.padding,
            spacing: init.spacing,
            button_height: init.button_height,
            button_width: init.button_width,
            browsers: init.browsers,
            default_for_domain: false,
            files: Vec::new(),
            is_domain_toggle_visible: false,
            current_uri: None,
            current_domain: None,
            settings: settings.clone(),
            show_keyboard_shortcuts_tooltips: false,
            show_full_url: settings.get("show-full-url"),
            default_for_domain_toggle_label: domain_label,
            menu_label,
            activate_menu: false,
            tracker: 0,
            buttons: browser_buttons,
            hotkeys,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        self.reset();
        match message {
            AppInputMessage::BrowserButtonPressed(number) => {
                if number < self.browsers.len() {
                    if let Some(file) = self.files.pop() {
                        let _ = self.browsers[number].launch(&[file.clone()], None::<&gio::AppLaunchContext>);
                        if self.default_for_domain {
                            let domain_part = file.uri().split('/').take(3).collect::<Vec<&str>>().join("/");
                            let mut defaults: HashMap<String, Vec<String>> = self.settings.get("browsers-default-for-domains");
                            if let Some(domains) = defaults.get_mut::<String>(&self.browsers[number].name().into()) {
                                domains.push(domain_part);
                            } else {
                                defaults.insert(self.browsers[number].name().into(), vec![domain_part]);
                            }
                            let _ = self.settings.set("browsers-default-for-domains", defaults);
                        }
                        if self.files.is_empty() {
                            sender.input(Self::Input::Quit);
                        } else {
                            sender.input(Self::Input::CurrentFileChanged);
                        }
                    } else {
                        let _ = self.browsers[number].launch(&[], None::<&gio::AppLaunchContext>);
                        sender.input(Self::Input::Quit);
                    }
                }
            }
            AppInputMessage::DomainToggleToggled(is_toggled) => {
                self.default_for_domain = is_toggled;
            }
            AppInputMessage::FilesOpenRequested(files) => {
                let defaults: HashMap<String, Vec<String>> = self.settings.get("browsers-default-for-domains");
                let mut new_files = Vec::new();
                for file in files.iter() {
                    let mut found = false;
                    if file.uri_scheme().is_some_and(|scheme| scheme == "http" || scheme == "https") {
                        let domain_part = file.uri().split('/').take(3).collect::<Vec<&str>>().join("/");
                        for browser in self.browsers.iter() {
                            if defaults.get::<String>(&browser.name().into()).is_some_and(|v| v.contains(&domain_part.clone())) {
                                let _ = browser.launch(&[file.clone()], None::<&gio::AppLaunchContext>);
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        new_files.push(file.clone());
                    }
                }
                if new_files.is_empty() {
                    sender.input(Self::Input::Quit);
                } else {
                    self.files = new_files;
                    sender.input(Self::Input::CurrentFileChanged);
                }
            }
            AppInputMessage::CurrentFileChanged => {
                if let Some(file) = self.files.last() {
                    self.current_uri = Some(file.uri().into());
                    if file.uri_scheme().is_some_and(|scheme| scheme == "http" || scheme == "https") {
                        self.current_domain = Some(file.uri().split('/').take(3).collect::<Vec<&str>>().join("/"));
                        self.is_domain_toggle_visible = true;
                    }
                }
            }
            AppInputMessage::KeyPressed(key) => {
                match key {
                    gtk::gdk::Key::q => sender.input(Self::Input::Quit),
                    gtk::gdk::Key::s => sender.input(Self::Input::ShowFullUrlToggleToggled(!self.show_full_url)),
                    gtk::gdk::Key::h => {
                        self.set_show_keyboard_shortcuts_tooltips(!self.show_keyboard_shortcuts_tooltips);
                        *APP_STATE.write() = self.show_keyboard_shortcuts_tooltips;
                    },
                    gtk::gdk::Key::d => {
                        self.default_for_domain = !self.default_for_domain;
                    },
                    gtk::gdk::Key::m => {
                        self.set_activate_menu(!self.activate_menu)
                    },
                    key => {
                        if let Some(key_upper) = key.to_upper().to_unicode() {
                            if let Some(key_num) = self.hotkeys.iter().position(|k| k == &key_upper) {
                                if key_num < self.browsers.len() {
                                    sender.input(Self::Input::BrowserButtonPressed(key_num));
                                }
                            }
                        }
                    }
                }
            },
            AppInputMessage::Quit => {
                relm4::main_application().quit();
            }
            AppInputMessage::ShowFullUrlToggleToggled(is_toggled) => {
                self.show_full_url = is_toggled;
                let _ = self.settings.set("show-full-url", is_toggled);
            }
            AppInputMessage::MenuOpened => {
                self.set_activate_menu(false)
            }
        }
    }

}

static BASE_BROKER: relm4::MessageBroker<AppInputMessage> = relm4::MessageBroker::new();

fn main() {
    let gtk_app = adw::Application::builder()
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .application_id("com.Nosterx.BrowDi")
        .build();

    let sender = BASE_BROKER.sender();

    gtk_app.connect_open(
        gtk::glib::clone!(@strong sender => move |application, files, _hint| {
            sender.send(AppInputMessage::FilesOpenRequested(files.to_vec())).unwrap();
            application.activate();
        }),
    );

    let app = RelmApp::from_app(gtk_app).with_broker(&BASE_BROKER);
    app.run::<BrowDiModel>(BrowDiInit::default());
}
