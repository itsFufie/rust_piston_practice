use opengl_graphics::{Filter, GlyphCache, TextureSettings};


static FONT: &str = "~/projects/rust_calculator/assets/static/NotoSerif-Medium.ttf";

pub fn load_text() {
    
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);

    let mut glyphs = GlyphCache::new(FONT, (), texture_settings)
        .expect(&format!("failed to load font `{}`", FONT));

}
