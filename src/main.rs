// Package: rusty_game_of_life_with_bevy
// File: src/main.rs

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

const GRID_SIZE: usize = 50; // Size of the grid (50x50)
const CELL_SIZE: f32 = 10.0; // Size of each cell in pixels
const TICK_RATE: f32 = 0.1; // Time in seconds between updates
const CLICK_RADIUS: usize = 2; // Radius of influence for clicks

#[derive(Resource)]
struct Grid {
    size: usize,
    cells: Vec<bool>,
    prev_cells: Vec<bool>, // Tracks the previous state of cells
}

impl Grid {
    fn new(size: usize) -> Self {
        let total_cells = size * size;
        let cells = vec![false; total_cells]; // All cells start dead
        let prev_cells = vec![false; total_cells];
        Self {
            size,
            cells,
            prev_cells,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.size + x]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.cells[y * self.size + x] = value;
    }

    fn toggle(&mut self, x: usize, y: usize) {
        let index = y * self.size + x;
        self.cells[index] = !self.cells[index];
    }

    fn neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && ny >= 0
                    && nx < self.size as isize
                    && ny < self.size as isize
                    && self.get(nx as usize, ny as usize)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn step(&mut self) {
        let mut new_cells = self.cells.clone();
        for x in 0..self.size {
            for y in 0..self.size {
                let alive = self.get(x, y);
                let neighbors = self.neighbors(x, y);
                new_cells[y * self.size + x] = match (alive, neighbors) {
                    (true, 2) | (_, 3) => true, // Stay alive or become alive
                    _ => false,                 // Otherwise, die
                };
            }
        }
        self.prev_cells = self.cells.clone();
        self.cells = new_cells;
    }

    fn get_color(&self, x: usize, y: usize) -> Color {
        let current = self.cells[y * self.size + x];
        let previous = self.prev_cells[y * self.size + x];
        match (previous, current) {
            (false, true) => Color::GREEN,  // Newly alive
            (true, false) => Color::RED,    // Recently dead
            (true, true) => Color::WHITE,   // Alive
            (false, false) => Color::BLACK, // Dead
        }
    }
}

#[derive(Resource)]
struct TickTimer(Timer);

fn setup(mut commands: Commands, mut grid: ResMut<Grid>, windows: Query<&Window, With<PrimaryWindow>>) {
    info!("Setting up the game...");

    // Spawn the camera
    let camera_entity = commands.spawn(Camera2dBundle::default()).id();
    info!("Camera spawned with entity ID: {:?}", camera_entity);

    // Ensure the camera is properly aligned and scaled
    if let Ok(window) = windows.get_single() {
        commands.entity(camera_entity).insert(Transform {
            scale: Vec3::new(
                GRID_SIZE as f32 * CELL_SIZE / window.width(),
                GRID_SIZE as f32 * CELL_SIZE / window.height(),
                1.0,
            ),
            ..Default::default()
        });
    }

    // Create a random initial state for the grid
    let mut rng = rand::thread_rng();
    for x in 0..grid.size {
        for y in 0..grid.size {
            let alive = rng.gen_bool(0.2); // 20% chance for a cell to be alive
            grid.set(x, y, alive);
        }
    }
    info!("Initial grid state created with size: {}x{}", grid.size, grid.size);

    // Add a visual border to outline the interactive grid area
    let border_size = Vec2::new(GRID_SIZE as f32 * CELL_SIZE, GRID_SIZE as f32 * CELL_SIZE);
    info!("Calculated border size: {:?}", border_size);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(1.0, 0.0, 0.0, 1.0), // Solid red border
            custom_size: Some(border_size),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)), // High z-value
        ..Default::default()
    });
    info!("Border sprite spawned at position: Vec3(0.0, 0.0, 10.0)");
}

fn render_grid(grid: Res<Grid>, mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    // Despawn all previously rendered sprites
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // Render grid cells
    for x in 0..grid.size {
        for y in 0..grid.size {
            let color = grid.get_color(x, y);
            if color != Color::BLACK {
                let cell_position = Vec3::new(
                    x as f32 * CELL_SIZE - GRID_SIZE as f32 * CELL_SIZE / 2.0,
                    y as f32 * CELL_SIZE - GRID_SIZE as f32 * CELL_SIZE / 2.0,
                    0.0,
                );

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(CELL_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(cell_position),
                    ..Default::default()
                });
            }
        }
    }
}

fn handle_clicks(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut grid: ResMut<Grid>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                let grid_x = ((cursor_position.x / window.width()) * GRID_SIZE as f32).floor() as isize;
                let grid_y = ((1.0 - cursor_position.y / window.height()) * GRID_SIZE as f32).floor() as isize;

                // Ensure the click is within the grid boundaries
                if grid_x >= 0
                    && grid_y >= 0
                    && grid_x < GRID_SIZE as isize
                    && grid_y < GRID_SIZE as isize
                {
                    for dx in -(CLICK_RADIUS as isize)..=(CLICK_RADIUS as isize) {
                        for dy in -(CLICK_RADIUS as isize)..=(CLICK_RADIUS as isize) {
                            let nx = grid_x + dx;
                            let ny = grid_y + dy;
                            if nx >= 0
                                && ny >= 0
                                && nx < GRID_SIZE as isize
                                && ny < GRID_SIZE as isize
                            {
                                grid.toggle(nx as usize, ny as usize);
                            }
                        }
                    }
                    info!("Clicked grid position: ({}, {})", grid_x, grid_y);
                } else {
                    info!(
                        "Click outside grid bounds: ({}, {}), Grid size: {}",
                        grid_x, grid_y, GRID_SIZE
                    );
                }
            }
        }
    }
}

fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "FPS: 0".to_string(),
            TextStyle {
                font: asset_server.load("fonts/UniversCondensed.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_translation(Vec3::new(-250.0, 200.0, 20.0)), // Ensure z-value is above the border
        ..Default::default()
    });
}

fn update_fps_counter(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text>) {
    if let Some(fps) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.average())
    {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("FPS: {:.0}", fps);
        }
    }
}

fn update_grid(time: Res<Time>, mut timer: ResMut<TickTimer>, mut grid: ResMut<Grid>) {
    if timer.0.tick(time.delta()).just_finished() {
        grid.step();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Game of Life".to_string(),
                resolution: (1000.0, 720.0).into(), // Set the initial window size
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(Grid::new(GRID_SIZE))
        .insert_resource(TickTimer(Timer::from_seconds(
            TICK_RATE,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, update_grid)
        .add_systems(Update, render_grid)
        .add_systems(Update, handle_clicks)
        .add_systems(Update, update_fps_counter)
        .run();
}