use druid::{
    AppDelegate, AppLauncher, Command, Env, Event, LocalizedString, Menu, MenuItem,
    Point, Size, SysMods, Widget, WidgetExt, WindowDesc, WindowHandle, WindowId,
};
use lapce_data::{
    command::{LapceUICommand, LAPCE_UI_COMMAND},
    config::Config,
    data::{LapceData, LapceWindowData, LapceWindowLens},
    db::{TabsInfo, WindowInfo},
};

use crate::logging::override_log_levels;
use crate::window::LapceWindowNew;

pub fn build_window(data: &LapceWindowData) -> impl Widget<LapceData> {
    LapceWindowNew::new(data).lens(LapceWindowLens(data.window_id))
}

pub fn launch() {
    let mut log_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Off)
        .level_for("piet_wgpu", log::LevelFilter::Info);

    if let Some(log_file) = Config::log_file().and_then(|f| fern::log_file(f).ok()) {
        log_dispatch = log_dispatch.chain(log_file);
    }

    log_dispatch = override_log_levels(log_dispatch);
    let _ = log_dispatch.apply();

    let mut launcher = AppLauncher::new().delegate(LapceAppDelegate::new());
    let data = LapceData::load(launcher.get_external_handle());
    for (_window_id, window_data) in data.windows.iter() {
        let root = build_window(window_data);
        let window = WindowDesc::new_with_id(window_data.window_id, root)
            .title(LocalizedString::new("Lapce").with_placeholder("Lapce"))
            .show_titlebar(false)
            .window_size(window_data.size)
            .menu(|_, _, _| {
                Menu::new("Lapce").entry(
                    Menu::new("")
                        .entry(MenuItem::new("About Lapce"))
                        .separator()
                        .entry(
                            MenuItem::new("Hide Lapce")
                                .command(druid::commands::HIDE_APPLICATION)
                                .hotkey(SysMods::Cmd, "h"),
                        )
                        .separator()
                        .entry(
                            MenuItem::new("Quit Lapce")
                                .command(druid::commands::QUIT_APP)
                                .hotkey(SysMods::Cmd, "q"),
                        ),
                )
            })
            .set_position(window_data.pos);
        launcher = launcher.with_window(window);
    }

    let launcher = launcher.configure_env(|env, data| data.reload_env(env));
    launcher.launch(data).expect("launch failed");
}

struct LapceAppDelegate {}

impl LapceAppDelegate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LapceAppDelegate {
    fn default() -> Self {
        Self::new()
    }
}

impl AppDelegate<LapceData> for LapceAppDelegate {
    fn event(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _window_id: WindowId,
        event: druid::Event,
        data: &mut LapceData,
        _env: &Env,
    ) -> Option<Event> {
        if let Event::ApplicationWillTerminate = event {
            let _ = data.db.save_app(data);
            return None;
        }
        Some(event)
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut LapceData,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        if let Some(window) = data.windows.remove(&id) {
            for (_, tab) in window.tabs.iter() {
                let _ = data.db.save_workspace(tab);
            }
            data.db.save_last_window(&window);
        }
    }

    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &Command,
        data: &mut LapceData,
        _env: &Env,
    ) -> druid::Handled {
        if cmd.is(LAPCE_UI_COMMAND) {
            let command = cmd.get_unchecked(LAPCE_UI_COMMAND);
            if let LapceUICommand::NewWindow(from_window_id) = command {
                let (size, pos) = data
                    .windows
                    .get(from_window_id)
                    .map(|win| (win.size, win.pos + (50.0, 50.0)))
                    .unwrap_or((Size::new(800.0, 600.0), Point::new(0.0, 0.0)));
                let info = WindowInfo {
                    size,
                    pos,
                    tabs: TabsInfo {
                        active_tab: 0,
                        workspaces: vec![],
                    },
                };
                let window_data = LapceWindowData::new(
                    data.keypress.clone(),
                    ctx.get_external_handle(),
                    &info,
                    data.db.clone(),
                );
                let root = build_window(&window_data);
                let window_id = window_data.window_id;
                data.windows.insert(window_id, window_data);
                let desc = WindowDesc::new_with_id(window_id, root)
                    .title(LocalizedString::new("Lapce").with_placeholder("Lapce"))
                    .show_titlebar(false)
                    .window_size(info.size)
                    .set_position(info.pos);
                ctx.new_window(desc);
                return druid::Handled::Yes;
            }
        }
        druid::Handled::No
    }

    fn window_added(
        &mut self,
        _id: WindowId,
        _handle: WindowHandle,
        _data: &mut LapceData,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }
}
