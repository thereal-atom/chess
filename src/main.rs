use macroquad::prelude::*;

#[macroquad::main("chess")]
async fn main() {
    let black_bishop = load_texture("assets/black-bishop.png").await.unwrap();

    fn get_color(color: i8) -> Color {
        let white = Color::new(0.760, 0.647, 0.568, 1.0);
        let black = Color::new(0.341, 0.223, 0.141, 1.0);

        if color == 1 { white } else { black }
    }

    loop {
        clear_background(BLACK);

        let tile_width = 64.0;

        let grid_width = 8;
        let grid_height = 8;

        for x in 0..grid_width {
            for y in 0..grid_height {
                let x_coord = screen_width() / 2.0 - tile_width * grid_width as f32 / 2.0 + tile_width * x as f32;
                let y_coord = screen_height() / 2.0 - tile_width * grid_height as f32 / 2.0 + tile_width * y as f32;

                draw_rectangle(
                    x_coord,
                    y_coord,
                    tile_width,
                    tile_width,
                    get_color(if x % 2 != y % 2 { 0 } else { 1 })
                );

                draw_texture(black_bishop, x_coord + tile_width / 2.0 - 20.0, y_coord + 4.0, WHITE);
            }
        }

        next_frame().await
    }
}