use macroquad::prelude::*;
mod utils;

#[macroquad::main("chess")]
async fn main() {
    fn draw_grid(grid_width: i32, grid_height: i32, tile_width: f32, tile_height: f32) {
        // let white = Color::new(0.760, 0.647, 0.568, 1.0);
        let white = Color::new(0.55, 0.48, 0.33, 1.0);
        let black = Color::new(0.341, 0.223, 0.141, 1.0);

        for x in 0..grid_width {
            for y in 0..grid_height {
                let x_coord = screen_width() / 2.0 - tile_width * grid_width as f32 / 2.0 + tile_width * x as f32;
                let y_coord = screen_height() / 2.0 - tile_height * grid_height as f32 / 2.0 + tile_height * y as f32;

                draw_rectangle(
                    x_coord,
                    y_coord,
                    tile_width,
                    tile_height,
                    if x % 2 != y % 2 { black } else { white }
                );
            };
        };
    }

    async fn load_position_from_fen(fen: String, starting_x: f32, starting_y: f32, tile_size: f32) {
        let black_bishop = load_texture("assets/bb.png").await.unwrap();
        let black_king = load_texture("assets/bk.png").await.unwrap();
        let black_knight = load_texture("assets/bn.png").await.unwrap();
        let black_pawn = load_texture("assets/bp.png").await.unwrap();
        let black_queen = load_texture("assets/bq.png").await.unwrap();
        let black_rook = load_texture("assets/br.png").await.unwrap();
    
        let white_bishop = load_texture("assets/wb.png").await.unwrap();
        let white_king = load_texture("assets/wk.png").await.unwrap();
        let white_knight = load_texture("assets/wn.png").await.unwrap();
        let white_pawn = load_texture("assets/wp.png").await.unwrap();
        let white_queen = load_texture("assets/wq.png").await.unwrap();
        let white_rook = load_texture("assets/wr.png").await.unwrap();

        let mut file = 0;
        let mut rank = 7;

        let position: Vec<&str> = fen.split(" ")
            .enumerate()
            .filter(|&(i, _)| i == 0)
            .map(|(_, e)| e)
            .collect();

        let position_symbols: Vec<char> = position[0]
            .chars()
            .collect();

        for symbol in position_symbols {
            if symbol == '/' {
                rank -= 1;
                file = 0;
            } else if symbol.is_numeric() {
                let skip = symbol
                    .to_digit(10)
                    .unwrap();

                if skip != 8 {
                    file += skip;
                };
            } else {
                let piece: Texture2D;

                match symbol {
                    'b' => piece = black_bishop,
                    'k' => piece = black_king,
                    'n' => piece = black_knight,
                    'p' => piece = black_pawn,
                    'q' => piece = black_queen,
                    'r' => piece = black_rook,
                    'B' => piece = white_bishop,
                    'K' => piece = white_king,
                    'N' => piece = white_knight,
                    'P' => piece = white_pawn,
                    'Q' => piece = white_queen,
                    'R' => piece = white_rook,
                    _ => piece = white_pawn,
                };

                draw_texture_ex(
                    piece,
                    starting_x + tile_size * file as f32,
                    starting_y + tile_size * (7 - rank) as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size:Some(vec2(tile_size, tile_size)),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None
                    }
                );

                file += 1;
            };
        };
    }

    loop {
        clear_background(Color::new(0.06, 0.07, 0.09, 1.0));

        let tile_size = 64.0;    

        draw_grid(8, 8, tile_size, tile_size);

        let starting_x = screen_width() / 2.0 - tile_size * 4.0;
        let starting_y = screen_height() / 2.0 - tile_size * 4.0;

        load_position_from_fen(
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            starting_x,
            starting_y,
            tile_size,
        ).await;

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{:?}", mouse_position());
        };

        next_frame().await;
    }
}