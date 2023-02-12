use bevy::{
    prelude::*,
    render::{camera::RenderTarget, render_graph::RenderGraph, Extract, RenderApp},
    window::{PresentMode, PrimaryWindow, WindowRef, WindowResolution},
};
use bevy_egui::{egui_node::EguiNode, EguiContext, EguiPlugin};

//TODO WindowId::new
//static SECOND_WINDOW_ID: Lazy<WindowId> = Lazy::new(WindowId::new);

#[derive(Resource)]
struct Images {
    bevy_icon: Handle<Image>,
}

type SecondaryWindowFilter = (With<Window>, Without<PrimaryWindow>);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<SharedUiState>()
        .add_startup_system(load_assets_system)
        .add_startup_system(create_new_window_system)
        .add_system(ui_first_window_system)
        .add_system(ui_second_window_system);

    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_system_to_schedule(ExtractSchedule, setup_second_window);

    app.run();
}

fn setup_second_window(
    second_window: Extract<Query<Entity, SecondaryWindowFilter>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let window = second_window.get_single().unwrap();
    if render_graph
        .get_node::<EguiNode>(SECONDARY_EGUI_PASS)
        .is_err()
    {
        bevy_egui::setup_pipeline(
            &mut render_graph,
            bevy_egui::RenderGraphConfig {
                window,
                egui_pass: SECONDARY_EGUI_PASS,
            },
        );
    }
}

const SECONDARY_EGUI_PASS: &str = "secondary_egui_pass";

fn create_new_window_system(mut commands: Commands) {
    // Spawn a second window
    let second_window_id = commands
        .spawn(Window {
            title: "Second window".to_owned(),
            resolution: WindowResolution::new(800., 600.),
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .id();

    // second window camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            target: RenderTarget::Window(WindowRef::Entity(second_window_id)),
            ..Default::default()
        },
        transform: Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn load_assets_system(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(Images {
        bevy_icon: assets.load("icon.png"),
    });
}

#[derive(Default)]
struct UiState {
    input: String,
}

#[derive(Default, Resource)]
struct SharedUiState {
    shared_input: String,
}

fn ui_first_window_system(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: Local<UiState>,
    mut shared_ui_state: ResMut<SharedUiState>,
    images: Res<Images>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let first_window = primary_window.get_single().unwrap();
    let bevy_texture_id = egui_context.add_image(images.bevy_icon.clone_weak());
    egui::Window::new("First Window").vscroll(true).show(
        egui_context.ctx_for_window_mut(first_window),
        |ui| {
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.input);
            });
            ui.horizontal(|ui| {
                ui.label("Shared input: ");
                ui.text_edit_singleline(&mut shared_ui_state.shared_input);
            });

            ui.add(egui::widgets::Image::new(bevy_texture_id, [256.0, 256.0]));
        },
    );
}

fn ui_second_window_system(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: Local<UiState>,
    mut shared_ui_state: ResMut<SharedUiState>,
    images: Res<Images>,
    secondary_window: Query<Entity, SecondaryWindowFilter>,
) {
    let second_window = secondary_window.get_single().unwrap();
    let bevy_texture_id = egui_context.add_image(images.bevy_icon.clone_weak());
    let ctx = match egui_context.try_ctx_for_window_mut(second_window) {
        Some(ctx) => ctx,
        None => return,
    };
    egui::Window::new("Second Window")
        .vscroll(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Write something else: ");
                ui.text_edit_singleline(&mut ui_state.input);
            });
            ui.horizontal(|ui| {
                ui.label("Shared input: ");
                ui.text_edit_singleline(&mut shared_ui_state.shared_input);
            });

            ui.add(egui::widgets::Image::new(bevy_texture_id, [256.0, 256.0]));
        });
}
