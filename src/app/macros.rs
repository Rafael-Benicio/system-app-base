#[macro_export]
macro_rules! frame_hate {
    ($a:expr) => {
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / $a));
    };
}

#[macro_export]
macro_rules! draw_simple_rect_image {
    ($canvas:expr,$txt_map:expr,$self:expr) => {{
        let color = $self.get_color().unwrap_or(Color {
            r: 0,
            g: 0,
            b: 0,
            a: u8::MAX,
        });

        if let Some(txt_name) = $self.get_texture_name() {
            if let Some(txr) = $txt_map.get_mut(txt_name) {
                txr.set_color_mod(color.r, color.g, color.b);
                let _ = $canvas.copy(&txr, None, *$self.get_draw_rect());
            }
        } else {
            $canvas.set_draw_color(color);
            let _ = $canvas.draw_rect(*$self.get_draw_rect());
            let _ = $canvas.fill_rect(*$self.get_draw_rect());
        }
    }};
}

#[macro_export]
macro_rules! draw_simple_rect {
    ($canvas:expr,$rect:expr,$color:expr) => {{
        $canvas.set_draw_color($color);
        let _ = $canvas.draw_rect($rect);
        let _ = $canvas.fill_rect($rect);
    }};
}

#[macro_export]
macro_rules! keydown {
    ($key:pat) => {
        KeyDown {
            keycode: Some($key),
            ..
        }
    };
}

#[macro_export]
macro_rules! keyup {
    ($key:pat) => {
        KeyUp {
            keycode: Some($key),
            ..
        }
    };
}
