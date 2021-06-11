use bevy::prelude::*;

pub struct Hud {
    pub top_left_text:String,
    pub top_right_text:String,
    pub center_text:String,
    pub foreground:Color
}

impl Hud {
    pub fn clear(&mut self) {
        self.top_left_text = "".into();
        self.top_right_text = "".into();
        self.center_text = "".into();
        self.foreground = Color::rgba(1.0,1.0,1.0,0.0)
    }
}

impl Default for Hud {
    fn default() -> Self {
        Self {
            top_left_text:"".into(),
            top_right_text:"".into(),
            center_text:"".into(),
            foreground:Color::rgba(1.0,1.0,1.0,0.0)
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum HudElement {
    TopLeft,
    Center, 
    TopRight,
    Foreground
}

fn hud_initialization_system(mut commands: Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>) {
    let font_size = 16.0;

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            //justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    }).with_children(|parent| {

        // center text
        parent.spawn_bundle(NodeBundle {
            style:Style {
                size:Size {
                    height:Val::Percent(100.0),
                    width:Val::Percent(100.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            material:materials.add((Color::NONE).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/default.ttf"),
                        font_size:font_size * 2.0,
                        color: Color::WHITE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            }).insert(HudElement::Center);
        });

        // top-left text
        parent.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "",
                TextStyle {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }).insert(HudElement::TopLeft);

        // right text
        parent.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "",
                TextStyle {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Right,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }).insert(HudElement::TopRight);

        // foreground color ontop of all text
        parent.spawn_bundle(NodeBundle {
            style:Style {
                size:Size {
                    height:Val::Percent(100.0),
                    width:Val::Percent(100.0),
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            material:materials.add(Color::rgba(1.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        }).insert(HudElement::Foreground);
    });
}

pub fn set_text(text:&mut Text, value:&str) {
    let section = text.sections.first_mut().expect("atleast one section in hud text was expected!");
    if section.value != value {
        section.value = value.into();
    }
}

fn update_text(hud:ResMut<Hud>, query:Query<(&mut Text, &HudElement)>) {
    if hud.is_changed() == false {
        return;
    }
    query.for_each_mut(|(mut text, element)| {
        match *element {
            HudElement::TopLeft => {
                set_text(&mut text, &hud.top_left_text);
            }
            HudElement::Center => {
                set_text(&mut text, &hud.center_text);
            }
            HudElement::TopRight => {
                set_text(&mut text, &hud.top_right_text);
            }
            _=>{}
        }
    });
}

fn update_foreground(hud:ResMut<Hud>, query:Query<(&mut Handle<ColorMaterial>, &HudElement, &Node)>, mut materials:ResMut<Assets<ColorMaterial>>) {
    query.for_each_mut(|(color_material_handle, element, _)| {
        if hud.is_changed() == false {
            return;
        }

        match *element {
            HudElement::Foreground => {
                if let Some(material) = materials.get_mut(color_material_handle.clone()) {
                    material.color = hud.foreground;
                }
            },
            _ => {}
        }
    });
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Hud::default());
        app.add_startup_system(hud_initialization_system.system());
        app.add_system(update_text.system());
        app.add_system(update_foreground.system());
        
    }
}