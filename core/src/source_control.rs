use std::{path::PathBuf, sync::Arc};

use druid::{
    kurbo::BezPath,
    piet::{
        LineCap, LineJoin, RoundFrom, StrokeStyle, Text,
        TextLayout as PietTextLayout, TextLayoutBuilder,
    },
    theme,
    widget::{CrossAxisAlignment, Flex, FlexParams, Label, Scroll},
    Affine, BoxConstraints, Color, Command, Cursor, Data, Env, Event, EventCtx,
    FontDescriptor, FontFamily, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Rect, RenderContext, Size, Target, TextLayout, UpdateCtx, Widget, WidgetExt,
    WidgetId, WidgetPod, WindowId,
};

use crate::{
    command::{
        CommandExecuted, CommandTarget, LapceCommand, LapceUICommand,
        LAPCE_UI_COMMAND,
    },
    config::LapceTheme,
    data::{FocusArea, LapceTabData, PanelKind},
    editor::{LapceEditorContainer, LapceEditorView},
    keypress::KeyPressFocus,
    movement::Movement,
    palette::svg_tree_size,
    panel::{LapcePanel, PanelHeaderKind, PanelPosition, PanelProperty},
    scroll::LapceScrollNew,
    split::{LapceSplitNew, SplitDirection, SplitMoveDirection},
    state::Mode,
    svg::file_svg_new,
    theme::OldLapceTheme,
};

pub const SOURCE_CONTROL_BUFFER: &'static str = "[Source Control Buffer]";
pub const SEARCH_BUFFER: &'static str = "[Search Buffer]";

#[derive(Clone)]
pub struct SourceControlData {
    pub active: WidgetId,
    pub widget_id: WidgetId,
    pub split_id: WidgetId,
    pub split_direction: SplitDirection,
    pub file_list_id: WidgetId,
    pub file_list_index: usize,
    pub editor_view_id: WidgetId,
    pub diff_files: Vec<(PathBuf, bool)>,
}

impl SourceControlData {
    pub fn new() -> Self {
        let file_list_id = WidgetId::next();
        let editor_view_id = WidgetId::next();
        Self {
            active: editor_view_id,
            widget_id: WidgetId::next(),
            editor_view_id,
            file_list_id,
            file_list_index: 0,
            split_id: WidgetId::next(),
            split_direction: SplitDirection::Horizontal,
            diff_files: Vec::new(),
        }
    }

    pub fn new_panel(&self, data: &LapceTabData) -> LapcePanel {
        let editor_data = data
            .main_split
            .editors
            .get(&data.source_control.editor_view_id)
            .unwrap();
        let input = LapceEditorView::new(editor_data)
            .hide_header()
            .hide_gutter()
            .set_placeholder("Commit Message".to_string())
            .padding(10.0);
        let content = SourceControlFileList::new(self.file_list_id);
        LapcePanel::new(
            PanelKind::SourceControl,
            self.widget_id,
            self.split_id,
            self.split_direction,
            PanelHeaderKind::Simple("Source Control".to_string()),
            vec![
                (
                    editor_data.view_id,
                    PanelHeaderKind::None,
                    input.boxed(),
                    Some(300.0),
                ),
                (
                    self.file_list_id,
                    PanelHeaderKind::Simple("Changes".to_string()),
                    content.boxed(),
                    None,
                ),
            ],
        )
    }
}

impl KeyPressFocus for SourceControlData {
    fn get_mode(&self) -> Mode {
        Mode::Normal
    }

    fn check_condition(&self, condition: &str) -> bool {
        match condition {
            "source_control_focus" => true,
            "list_focus" => self.active == self.file_list_id,
            _ => false,
        }
    }

    fn run_command(
        &mut self,
        ctx: &mut EventCtx,
        command: &LapceCommand,
        count: Option<usize>,
        env: &Env,
    ) -> CommandExecuted {
        match command {
            LapceCommand::SplitUp => {
                ctx.submit_command(Command::new(
                    LAPCE_UI_COMMAND,
                    LapceUICommand::SplitEditorMove(
                        SplitMoveDirection::Up,
                        self.active,
                    ),
                    Target::Widget(self.split_id),
                ));
            }
            LapceCommand::SourceControlCancel => {
                ctx.submit_command(Command::new(
                    LAPCE_UI_COMMAND,
                    LapceUICommand::FocusEditor,
                    Target::Auto,
                ));
            }
            LapceCommand::Up | LapceCommand::ListPrevious => {
                self.file_list_index = Movement::Up.update_index(
                    self.file_list_index,
                    self.diff_files.len(),
                    1,
                    true,
                );
            }
            LapceCommand::Down | LapceCommand::ListNext => {
                self.file_list_index = Movement::Down.update_index(
                    self.file_list_index,
                    self.diff_files.len(),
                    1,
                    true,
                );
            }
            LapceCommand::ListExpand => {
                if self.diff_files.len() > 0 {
                    self.diff_files[self.file_list_index].1 =
                        !self.diff_files[self.file_list_index].1;
                }
            }
            _ => return CommandExecuted::No,
        }
        CommandExecuted::Yes
    }

    fn receive_char(&mut self, ctx: &mut EventCtx, c: &str) {}
}

pub struct SourceControlFileList {
    widget_id: WidgetId,
    mouse_down: Option<usize>,
}

impl SourceControlFileList {
    pub fn new(widget_id: WidgetId) -> Self {
        Self {
            widget_id,
            mouse_down: None,
        }
    }

    pub fn request_focus(&self, ctx: &mut EventCtx, data: &mut LapceTabData) {
        ctx.request_focus();
        let source_control = Arc::make_mut(&mut data.source_control);
        source_control.active = self.widget_id;
        data.focus_area = FocusArea::Panel(PanelKind::SourceControl);
        data.focus = self.widget_id;
    }
}

impl Widget<LapceTabData> for SourceControlFileList {
    fn id(&self) -> Option<WidgetId> {
        Some(self.widget_id)
    }

    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut LapceTabData,
        env: &Env,
    ) {
        match event {
            Event::MouseMove(mouse_event) => {
                ctx.set_cursor(&druid::Cursor::Pointer);
                ctx.set_handled();
            }
            Event::MouseUp(mouse_event) => {
                let line_height = data.config.editor.line_height as f64;
                let y = mouse_event.pos.y;
                if y > 0.0 {
                    let line = (y / line_height).floor() as usize;
                    if line < data.source_control.diff_files.len()
                        && mouse_event.pos.x < line_height
                    {
                        if let Some(mouse_down) = self.mouse_down {
                            if mouse_down == line {
                                let source_control =
                                    Arc::make_mut(&mut data.source_control);
                                source_control.diff_files[line].1 =
                                    !source_control.diff_files[line].1;
                            }
                        }
                    }
                }
                self.mouse_down = None;
                ctx.set_handled();
            }
            Event::MouseDown(mouse_event) => {
                self.mouse_down = None;
                let source_control = Arc::make_mut(&mut data.source_control);
                let line_height = data.config.editor.line_height as f64;
                let y = mouse_event.pos.y;
                if y > 0.0 {
                    let line = (y / line_height).floor() as usize;
                    if line < source_control.diff_files.len() {
                        source_control.file_list_index = line;
                        if mouse_event.pos.x < line_height {
                            self.mouse_down = Some(line);
                        }
                    }
                }
                self.request_focus(ctx, data);
                ctx.set_handled();
            }
            Event::KeyDown(key_event) => {
                let mut keypress = data.keypress.clone();
                let mut source_control = data.source_control.clone();
                Arc::make_mut(&mut keypress).key_down(
                    ctx,
                    key_event,
                    Arc::make_mut(&mut source_control),
                    env,
                );

                data.keypress = keypress.clone();
                data.source_control = source_control.clone();
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(LAPCE_UI_COMMAND) => {
                let command = cmd.get_unchecked(LAPCE_UI_COMMAND);
                match command {
                    LapceUICommand::Focus => {
                        self.request_focus(ctx, data);
                        ctx.set_handled();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &LapceTabData,
        env: &Env,
    ) {
        match event {
            LifeCycle::FocusChanged(_) => {
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &LapceTabData,
        data: &LapceTabData,
        env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &LapceTabData,
        env: &Env,
    ) -> Size {
        let line_height = data.config.editor.line_height as f64;
        let height = line_height * data.source_control.diff_files.len() as f64;
        Size::new(bc.max().width, height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &LapceTabData, env: &Env) {
        let line_height = data.config.editor.line_height as f64;

        let files = &data.source_control.diff_files;

        if ctx.is_focused() && files.len() > 0 {
            let rect = Size::new(ctx.size().width, line_height)
                .to_rect()
                .with_origin(Point::new(
                    0.0,
                    data.source_control.file_list_index as f64 * line_height,
                ));
            ctx.fill(
                rect,
                data.config.get_color_unchecked(LapceTheme::PANEL_CURRENT),
            );
        }

        let rect = ctx.region().bounding_box();
        let start_line = (rect.y0 / line_height).floor() as usize;
        let end_line = (rect.y1 / line_height).ceil() as usize;
        for line in start_line..end_line {
            if line >= files.len() {
                break;
            }
            let y = line_height * line as f64;
            let (mut path, checked) = files[line].clone();
            if let Some(workspace) = data.workspace.as_ref() {
                path = path
                    .strip_prefix(&workspace.path)
                    .unwrap_or(&path)
                    .to_path_buf();
            }
            {
                let width = 13.0;
                let height = 13.0;
                let origin = Point::new(
                    (line_height - width) / 2.0 + 5.0,
                    (line_height - height) / 2.0 + y,
                );
                let rect = Size::new(width, height).to_rect().with_origin(origin);
                ctx.stroke(rect, &Color::rgb8(0, 0, 0), 1.0);

                if checked {
                    let mut path = BezPath::new();
                    path.move_to((origin.x + 3.0, origin.y + 7.0));
                    path.line_to((origin.x + 6.0, origin.y + 9.5));
                    path.line_to((origin.x + 10.0, origin.y + 3.0));
                    ctx.stroke(path, &Color::rgb8(0, 0, 0), 2.0);
                }
            }
            let svg = file_svg_new(&path);
            let width = 13.0;
            let height = 13.0;
            let rect = Size::new(width, height).to_rect().with_origin(Point::new(
                (line_height - width) / 2.0 + line_height,
                (line_height - height) / 2.0 + y,
            ));
            ctx.draw_svg(&svg, rect, None);

            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            let text_layout = ctx
                .text()
                .new_text_layout(file_name)
                .font(FontFamily::SYSTEM_UI, 13.0)
                .text_color(
                    data.config
                        .get_color_unchecked(LapceTheme::EDITOR_FOREGROUND)
                        .clone(),
                )
                .build()
                .unwrap();
            ctx.draw_text(&text_layout, Point::new(line_height * 2.0, y + 4.0));
            let folder = path
                .parent()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if folder != "" {
                let x = text_layout.size().width;

                let text_layout = ctx
                    .text()
                    .new_text_layout(folder)
                    .font(FontFamily::SYSTEM_UI, 13.0)
                    .text_color(
                        data.config
                            .get_color_unchecked(LapceTheme::EDITOR_DIM)
                            .clone(),
                    )
                    .build()
                    .unwrap();
                ctx.draw_text(
                    &text_layout,
                    Point::new(line_height * 2.0 + x + 5.0, y + 4.0),
                );
            }
        }
    }
}
