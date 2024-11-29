use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{self as ui};
use macroquad_particles::{self as particles};

mod presets;

struct SubConfig {
    emission_rect_width: f32,
    emission_rect_height: f32,
    emission_sphere_radius: f32,
    rectangle_aspect_ratio: f32,
    circle_subdivisions: u32,
    size_curve: particles::Curve,
}

impl SubConfig {
    fn new() -> Self {
        Self {
            emission_rect_width: 0.0,
            emission_rect_height: 0.0,
            emission_sphere_radius: 0.0,
            rectangle_aspect_ratio: 1.0,
            circle_subdivisions: 30,
            size_curve: particles::Curve {
                points: vec![(0.0, 1.0), (1.0, 1.0)],
                interpolation: particles::Interpolation::Linear,
                resolution: 100,
            },
        }
    }
}

struct ParticlesEditor {
    emitter: particles::Emitter,
    coords: Vec2,
    sub_config: SubConfig,
}

impl ParticlesEditor {
    fn new() -> Self {
        let emitter = particles::Emitter::new(particles::EmitterConfig {
            ..Default::default()
        });

        let coords = vec2(screen_width() / 2.0, screen_height() / 2.0);

        let sub_config = SubConfig::new();

        Self {
            emitter,
            coords,
            sub_config,
        }
    }

    fn update_coords(&mut self) {
        self.coords = vec2(screen_width() / 2.0, screen_height() / 2.0);
    }

    fn draw_emitter(&mut self) {
        self.emitter.draw(self.coords);
    }
}

struct WindowResizeDetector {
    last_size: Vec2,
}

impl WindowResizeDetector {
    fn new() -> Self {
        Self {
            last_size: vec2(screen_width(), screen_height()),
        }
    }

    fn update(&mut self) {
        self.last_size = vec2(screen_width(), screen_height());
    }

    fn has_resized(&self) -> bool {
        screen_width() != self.last_size.x || screen_height() != self.last_size.y
    }
}

fn color_picker_texture(w: usize, h: usize) -> (Texture2D, Image) {
    let ratio = 1.0 / h as f32;

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
    let image_data = image.get_image_data_mut();

    for j in 0..h {
        for i in 0..w {
            let lightness = 1.0 - i as f32 * ratio;
            let hue = j as f32 * ratio;

            image_data[i + j * w] = macroquad::color::hsl_to_rgb(hue, 1.0, lightness).into();
        }
    }

    (Texture2D::from_image(&image), image)
}

fn color_picker(
    ui: &mut macroquad::ui::Ui,
    id: macroquad::ui::Id,
    data: &mut Color,
    color_picker_texture: &Texture2D,
) -> bool {
    let is_mouse_captured = ui.is_mouse_captured();

    let mut canvas = ui.canvas();
    let cursor = canvas.request_space(Vec2::new(200., 220.));
    let mouse = mouse_position();

    let x = mouse.0 as i32 - cursor.x as i32;
    let y = mouse.1 as i32 - (cursor.y as i32 + 20);

    if x > 0 && x < 200 && y > 0 && y < 200 {
        let ratio = 1.0 / 200.0 as f32;
        let lightness = 1.0 - x as f32 * ratio;
        let hue = y as f32 * ratio;

        if is_mouse_button_down(MouseButton::Left) && is_mouse_captured == false {
            *data = macroquad::color::hsl_to_rgb(hue, 1.0, lightness).into();
        }
    }

    canvas.rect(
        Rect::new(cursor.x - 5.0, cursor.y - 5.0, 210.0, 395.0),
        Color::new(0.7, 0.7, 0.7, 1.0),
        Color::new(0.9, 0.9, 0.9, 1.0),
    );

    canvas.rect(
        Rect::new(cursor.x, cursor.y, 200.0, 18.0),
        Color::new(0.0, 0.0, 0.0, 1.0),
        Color::new(data.r, data.g, data.b, 1.0),
    );

    canvas.image(
        Rect::new(cursor.x, cursor.y + 20.0, 200.0, 200.0),
        color_picker_texture,
    );

    let (h, _, l) = macroquad::color::rgb_to_hsl(*data);

    canvas.rect(
        Rect::new(
            cursor.x + (1.0 - l) * 200.0 - 3.5,
            cursor.y + h * 200. + 20.0 - 3.5,
            7.0,
            7.0,
        ),
        Color::new(0.3, 0.3, 0.3, 1.0),
        Color::new(1.0, 1.0, 1.0, 1.0),
    );

    ui.slider(hash!(id, "alpha"), "Alpha", 0.0..1.0, &mut data.a);

    ui.separator();

    ui.slider(hash!(id, "red"), "Red", 0.0..1.0, &mut data.r);
    ui.slider(hash!(id, "green"), "Green", 0.0..1.0, &mut data.g);
    ui.slider(hash!(id, "blue"), "Blue", 0.0..1.0, &mut data.b);

    ui.separator();

    let (mut h, mut s, mut l) = macroquad::color::rgb_to_hsl(*data);

    ui.slider(hash!(id, "hue"), "Hue", 0.0..1.0, &mut h);
    ui.slider(hash!(id, "saturation"), "Saturation", 0.0..1.0, &mut s);
    ui.slider(hash!(id, "lightess"), "Lightness", 0.0..1.0, &mut l);

    let Color { r, g, b, .. } = macroquad::color::hsl_to_rgb(h, s, l);
    data.r = r;
    data.g = g;
    data.b = b;

    ui.separator();
    if ui.button(None, "    ok    ")
        || is_key_down(KeyCode::Escape)
        || is_key_down(KeyCode::Enter)
        || (is_mouse_button_pressed(MouseButton::Left)
            && Rect::new(cursor.x - 10., cursor.y - 10.0, 230., 420.)
                .contains(vec2(mouse.0, mouse.1))
                == false)
    {
        return true;
    }

    false
}

fn colorbox(
    ui: &mut macroquad::ui::Ui,
    id: macroquad::ui::Id,
    label: &str,
    data: &mut Color,
    color_picker_texture: &Texture2D,
) {
    ui.label(None, label);
    let mut canvas = ui.canvas();
    let cursor = canvas.cursor();

    canvas.rect(
        Rect::new(cursor.x + 20.0, cursor.y, 50.0, 18.0),
        Color::new(0.2, 0.2, 0.2, 1.0),
        Color::new(data.r, data.g, data.b, 1.0),
    );

    if ui.last_item_clicked() {
        *ui.get_bool(hash!(id, "color picker opened")) ^= true;
    }

    if *ui.get_bool(hash!(id, "color picker opened")) {
        ui.popup(hash!(id, "color popup"), Vec2::new(200., 400.), |ui| {
            if color_picker(ui, id, data, &color_picker_texture) {
                *ui.get_bool(hash!(id, "color picker opened")) = false;
            }
        });
    }
}

fn curvebox(ui: &mut macroquad::ui::Ui, curve: &mut particles::Curve) {
    let mut canvas = ui.canvas();
    let w = 200.0;
    let h = 50.0;
    let min = 0.0;
    let max = 2.0;
    let (mouse_x, mouse_y) = mouse_position();
    let pos = canvas.request_space(Vec2::new(w, h));

    canvas.rect(
        Rect::new(pos.x, pos.y, w, h),
        Color::new(0.5, 0.5, 0.5, 1.0),
        None,
    );

    let t = ((mouse_x - pos.x) / w).max(0.0).min(1.0);

    for (_, line) in curve.points.windows(2).enumerate() {
        let (x0, value0) = line[0];
        let (x1, value1) = line[1];
        let y0 = (1.0 - value0 / (max - min)) * h;
        let y1 = (1.0 - value1 / (max - min)) * h;

        canvas.line(
            Vec2::new(pos.x + x0 * w, pos.y + y0),
            Vec2::new(pos.x + x1 * w, pos.y + y1),
            Color::new(0.5, 0.5, 0.5, 1.0),
        );
    }
    for (x, value) in &curve.points {
        let y = (1.0 - value / (max - min)) * h;

        let color = if (x - t).abs() < 0.1 {
            Color::new(0.9, 0.5, 0.5, 1.0)
        } else {
            Color::new(0.5, 0.5, 0.5, 1.0)
        };
        canvas.rect(
            Rect::new(pos.x + x * w - 2., pos.y + y - 2., 4., 4.),
            color,
            color,
        );
    }

    if is_mouse_button_down(MouseButton::Left) {
        let rect = Rect::new(pos.x, pos.y, w, h);

        let new_value = ((1.0 - (mouse_y - pos.y) / h) * (max - min))
            .min(max)
            .max(min);
        let dragging_point = ui.get_any::<Option<usize>>(hash!("dragging point"));

        if let Some(ix) = dragging_point {
            let (x, value) = curve.points.get_mut(*ix).unwrap();
            *x = t;
            *value = new_value;
        } else {
            if rect.contains(vec2(mouse_x, mouse_y)) {
                let closest_point = curve
                    .points
                    .iter_mut()
                    .position(|(x, _)| (*x - t).abs() < 0.1);

                if let Some(ix) = closest_point {
                    let (_, value) = curve.points.get_mut(ix).unwrap();
                    *value = new_value;
                    *ui.get_any::<Option<usize>>(hash!("dragging point")) = Some(ix);
                } else {
                    curve.points.push((t, new_value));
                    curve
                        .points
                        .sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
                }
            }
        }
    } else {
        *ui.get_any::<Option<usize>>(hash!("dragging point")) = None;
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Particle Editor".to_owned(),
        window_width: 1000,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut editor = ParticlesEditor::new();
    let mut resizer_detector = WindowResizeDetector::new();

    let color_picker_texture = color_picker_texture(200, 200).0;

    loop {
        clear_background(BLACK);

        if resizer_detector.has_resized() {
            resizer_detector.update();
            editor.update_coords();
        }

        ui::widgets::Window::new(hash!(), vec2(5.0, 5.0), vec2(350.0, 790.0))
            .label("Config")
            .ui(&mut ui::root_ui(), |ui| {
                // emitting: bool,
                ui.checkbox(hash!(), "Emitting", &mut editor.emitter.config.emitting);

                // local_coords: bool,
                ui.checkbox(
                    hash!(),
                    "Local coords",
                    &mut editor.emitter.config.local_coords,
                );

                // one_shot: bool,
                ui.checkbox(hash!(), "One shot", &mut editor.emitter.config.one_shot);

                // amount: u32,
                ui.drag(hash!(), "Amount", None, &mut editor.emitter.config.amount);

                ui.separator();

                // Time Config
                ui.tree_node(hash!(), "Time", |ui| {
                    // lifetime: f32,
                    ui.drag(
                        hash!(),
                        "Lifetime",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.lifetime,
                    );
                    // lifetime_randomness: f32,
                    ui.drag(
                        hash!(),
                        "Lifetime randomness",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.lifetime_randomness,
                    );
                    // explosiveness: f32,
                    ui.drag(
                        hash!(),
                        "Explosiveness",
                        (0.0, 1.0),
                        &mut editor.emitter.config.explosiveness,
                    );
                });

                ui.separator();

                // Shape Config
                ui.tree_node(hash!(), "Shape", |ui| {
                    // shape: ParticleShape,
                    let mut shape = match editor.emitter.config.shape {
                        particles::ParticleShape::Rectangle { .. } => 0,
                        particles::ParticleShape::Circle { .. } => 1,
                        particles::ParticleShape::CustomMesh { .. } => 2,
                    };
                    let old_shape = shape;
                    ui.combo_box(hash!(), "Shape", &["Rectangle", "Circle"], &mut shape);
                    match shape {
                        0 => {
                            editor.emitter.config.shape = particles::ParticleShape::Rectangle {
                                aspect_ratio: editor.sub_config.rectangle_aspect_ratio,
                            };
                            let old_aspect_ratio = editor.sub_config.rectangle_aspect_ratio;
                            ui.drag(
                                hash!(),
                                "Rectangle aspect ratio",
                                (0.0, f32::INFINITY),
                                &mut editor.sub_config.rectangle_aspect_ratio,
                            );
                            if old_aspect_ratio != editor.sub_config.rectangle_aspect_ratio {
                                editor.emitter.update_particle_mesh();
                            }
                        }
                        1 => {
                            editor.emitter.config.shape = particles::ParticleShape::Circle {
                                subdivisions: editor.sub_config.circle_subdivisions,
                            };
                            let old_subdivisions = editor.sub_config.circle_subdivisions;
                            ui.drag(
                                hash!(),
                                "Circle subdivisions",
                                (0, u32::MAX),
                                &mut editor.sub_config.circle_subdivisions,
                            );
                            if old_subdivisions != editor.sub_config.circle_subdivisions {
                                editor.emitter.update_particle_mesh();
                            }
                        }
                        2 => {
                            // Set shape to rectangle if it is a custom mesh
                            editor.emitter.config.shape =
                                particles::ParticleShape::Rectangle { aspect_ratio: 1.0 };
                        }
                        _ => unreachable!(),
                    };
                    if old_shape != shape {
                        editor.emitter.update_particle_mesh();
                    }

                    // emission_shape: EmissionShape,
                    let mut emission_shape = match editor.emitter.config.emission_shape {
                        particles::EmissionShape::Point => 0,
                        particles::EmissionShape::Rect { .. } => 1,
                        particles::EmissionShape::Sphere { .. } => 2,
                    };
                    ui.combo_box(
                        hash!(),
                        "Emission shape",
                        &["Point", "Rect", "Sphere"],
                        &mut emission_shape,
                    );
                    match emission_shape {
                        0 => editor.emitter.config.emission_shape = particles::EmissionShape::Point,
                        1 => {
                            editor.emitter.config.emission_shape = particles::EmissionShape::Rect {
                                width: editor.sub_config.emission_rect_width,
                                height: editor.sub_config.emission_rect_height,
                            };
                            ui.drag(
                                hash!(),
                                "Rect width",
                                (0.0, f32::INFINITY),
                                &mut editor.sub_config.emission_rect_width,
                            );
                            ui.drag(
                                hash!(),
                                "Rect height",
                                (0.0, f32::INFINITY),
                                &mut editor.sub_config.emission_rect_height,
                            );
                        }
                        2 => {
                            editor.emitter.config.emission_shape =
                                particles::EmissionShape::Sphere {
                                    radius: editor.sub_config.emission_sphere_radius,
                                };
                            ui.drag(
                                hash!(),
                                "Sphere radius",
                                (0.0, f32::INFINITY),
                                &mut editor.sub_config.emission_sphere_radius,
                            );
                        }
                        _ => unreachable!(),
                    }
                });

                ui.separator();

                // Direction Config
                ui.tree_node(hash!(), "Direction", |ui| {
                    // initial_direction: Vec2,
                    ui.drag(
                        hash!(),
                        "Initial direction x",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.initial_direction.x,
                    );
                    ui.drag(
                        hash!(),
                        "Initial direction y",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.initial_direction.y,
                    );
                    // initial_direction_spread: f32,
                    ui.drag(
                        hash!(),
                        "Initial direction spread",
                        (0.0, 2.0 * std::f32::consts::PI),
                        &mut editor.emitter.config.initial_direction_spread,
                    );
                    // gravity: Vec2,
                    ui.drag(
                        hash!(),
                        "Gravity x",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.gravity.x,
                    );
                    ui.drag(
                        hash!(),
                        "Gravity y",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.gravity.y,
                    );
                });

                ui.separator();

                // Velocity Config
                ui.tree_node(hash!(), "Velocity", |ui| {
                    // initial_velocity: f32,
                    ui.drag(
                        hash!(),
                        "Initial velocity",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.initial_velocity,
                    );
                    // initial_velocity_randomness: f32,
                    ui.drag(
                        hash!(),
                        "Initial velocity randomness",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.initial_velocity_randomness,
                    );
                    // linear_accel: f32,
                    ui.drag(
                        hash!(),
                        "Linear accel",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.linear_accel,
                    );
                });

                ui.separator();

                // Angle Config
                ui.tree_node(hash!(), "Angle", |ui| {
                    // initial_rotation: f32,
                    ui.drag(
                        hash!(),
                        "Initial rotation",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.initial_rotation,
                    );

                    // initial_rotation_randomness: f32,
                    ui.drag(
                        hash!(),
                        "Initial rotation randomness",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.initial_rotation_randomness,
                    );
                    // initial_angular_velocity: f32,
                    ui.drag(
                        hash!(),
                        "Initial angular velocity",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.initial_angular_velocity,
                    );
                    // initial_angular_velocity_randomness: f32,
                    ui.drag(
                        hash!(),
                        "Initial angular velocity randomness",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.initial_angular_velocity_randomness,
                    );
                    // angular_accel: f32,
                    ui.drag(
                        hash!(),
                        "Angular accel",
                        (-f32::INFINITY, f32::INFINITY),
                        &mut editor.emitter.config.angular_accel,
                    );
                    // angular_damping: f32,
                    ui.drag(
                        hash!(),
                        "Angular damping",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.angular_damping,
                    );
                });

                ui.separator();

                // Size Config
                ui.tree_node(hash!(), "Size", |ui| {
                    // size: f32,
                    ui.drag(
                        hash!(),
                        "Size",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.size,
                    );
                    // size_randomness: f32,
                    ui.drag(
                        hash!(),
                        "Size randomness",
                        (0.0, f32::INFINITY),
                        &mut editor.emitter.config.size_randomness,
                    );
                    // size_curve: Option<Curve>,
                    let mut size_curve_enabled = editor.emitter.config.size_curve.is_some();
                    ui.checkbox(hash!(), "Size curve", &mut size_curve_enabled);
                    if size_curve_enabled {
                        let size_curve = editor
                            .emitter
                            .config
                            .size_curve
                            .get_or_insert(editor.sub_config.size_curve.clone());
                        curvebox(ui, size_curve);
                        editor.emitter.rebuild_size_curve();
                    } else {
                        editor.emitter.config.size_curve = None;
                        editor.emitter.rebuild_size_curve();
                    }
                });

                // Color Config
                ui.tree_node(hash!(), "Color", |ui| {
                    // blend_mode: BlendMode,

                    // colors_curve: ColorCurve,
                    let curve = &mut editor.emitter.config.colors_curve;
                    colorbox(
                        ui,
                        hash!(),
                        "Start",
                        &mut curve.start,
                        &color_picker_texture,
                    );
                    colorbox(ui, hash!(), "Mid", &mut curve.mid, &color_picker_texture);
                    colorbox(ui, hash!(), "End", &mut curve.end, &color_picker_texture);
                });

                // texture: Option<Texture2D>,
                //
                // atlas: Option<AtlasConfig>,
                //
                // material: Option<ParticleMaterial>,
                //
                // post_processing: Option<PostProcessing>,

                ui.tree_node(hash!(), "Presets", |ui| {
                    if ui.button(None, "Default") {
                        editor.emitter.config = presets::default();
                        editor.sub_config = SubConfig::new();
                        editor.emitter.update_particle_mesh();
                    }
                    if ui.button(None, "Smoke") {
                        editor.emitter.config = presets::smoke();
                        editor.sub_config = SubConfig::new();
                        editor.emitter.update_particle_mesh();
                    }
                    if ui.button(None, "Fire") {
                        editor.emitter.config = presets::fire();
                        editor.sub_config = SubConfig::new();
                        editor.emitter.update_particle_mesh();
                    }
                    if ui.button(None, "Explosion") {
                        editor.emitter.config = presets::explosion();
                        editor.sub_config = SubConfig::new();
                        editor.emitter.update_particle_mesh();
                    }
                });

                if ui.button(None, "Reset") {
                    editor.emitter.config = particles::EmitterConfig {
                        ..Default::default()
                    };
                    editor.sub_config = SubConfig::new();
                    editor.emitter.update_particle_mesh();
                }
                if ui.button(None, "Log config") {
                    println!("{:#?}", editor.emitter.config);
                }
            });

        editor.draw_emitter();

        next_frame().await
    }
}
