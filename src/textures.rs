use piston_window::*;

pub fn load_textures(window: &mut PistonWindow) -> (G2dTexture, G2dTexture, G2dTexture, G2dTexture, G2dTexture) {
    let cell_white = Texture::from_path(
        &mut window.create_texture_context(),
        "./img/cell_white.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let cell_blue = Texture::from_path(
        &mut window.create_texture_context(),
        "./img/cell_blue.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let cell_yellow = Texture::from_path(
        &mut window.create_texture_context(),
        "./img/cell_yellow.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let piece_black = Texture::from_path(
        &mut window.create_texture_context(),
        "./img/black.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    
    let piece_white = Texture::from_path(
        &mut window.create_texture_context(),
        "./img/white.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    return (cell_white, cell_blue, cell_yellow, piece_black, piece_white);
}