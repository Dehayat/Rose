use sdl2::{self, render::{Canvas, TextureCreator, Texture}, video::{Window, WindowContext},pixels::Color, image::LoadTexture};

pub struct Renderer{
    canvas:Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    //textures:Vec<Texture<'a>>,
}
impl Renderer{
    pub fn new(window:Window) -> Renderer {
        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let tex = canvas.texture_creator();
        //let textures: Vec<Texture<'a>> = vec![];
        Renderer{
            canvas,
            texture_creator: tex,
            //textures: textures,
        }
    }
    // pub fn test(&'a mut self) {
    //     let text = self.texture_creator.load_texture("rose_engine_logo").unwrap();
    //     self.textures.push(text);
    // }

    pub fn render(&mut self) {
        //self.canvas.copy(&self.textures[0], None, None).unwrap();
        self.canvas.set_draw_color(Color::RGB(20, 20, 20));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 210, 0));
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(10, 10, 100, 200))
            .unwrap();

        self.canvas.present();
    }
}