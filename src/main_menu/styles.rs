use bevy::prelude::*;


pub const TRY_COLOR: Color = Color::RED;
pub const TRY_COLOR2: Color = Color::BLUE;
pub const TRY_COLOR3: Color = Color::GREEN;
pub const TRY_COLOR4: Color = Color::YELLOW;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MAIN_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(20.0), Val::Px(20.0)),

    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const TITLE_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(50.0), Val::Px(8.0)),

    ..Style::DEFAULT
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}

pub const BOTTOM_HOLDER: Style = Style{
    flex_direction: FlexDirection::Row,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),    
    justify_content: JustifyContent::SpaceEvenly,
    ..Style::DEFAULT
}; 

pub const IMAGE_HOLDER: Style = Style{
    flex_direction: FlexDirection::ColumnReverse,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),    
   
    ..Style::DEFAULT
}; 
pub const IMAGE_STYLE: Style = Style {
    justify_content:JustifyContent::FlexEnd,
    //1920x1080 
    size: Size::new(Val::Px(1344.0), Val::Px(756.0)),    

    margin: UiRect::new(Val::Percent(1.0), Val::Percent(2.0), Val::Px(0.0), Val::Percent(1.0)),
    ..Style::DEFAULT
};


pub const TEXT_MENU: Style = Style{
    flex_direction: FlexDirection::Column,
    justify_content:JustifyContent::Center,
    size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),    

    ..Style::DEFAULT
}; 