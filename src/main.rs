use macroquad::prelude::*;

#[macroquad::main("RustGround")]
async fn main() {
    // Параметры персонажа
    let mut pos = vec2(200.0, 300.0);
    let mut size = vec2(32.0, 32.0);
    let mut velocity = vec2(0.0, 0.0);
    let mut on_ground = false;
    let mut score = 0;
    // Параметры физики
    let mut speed = 250.0;
    let mut gravity = 900.0;
    let mut jump_force = -420.0;

    // Пол (плейсхолдер), x, y, ширина, высота
    let floors = vec![
        Rect::new(0.0, 400.0, 400.0, 200.0),
        Rect::new(600.0, 400.0, 400.0, 200.0),
    ];

    //Кнопка
    let button_rect = Rect::new(150.0, 100.0, 100.0, 100.0);

    // Особенности
    let mut d_jump_count = 0;

    loop {
        // игровой цикл
        let dt = get_frame_time().min(0.05); // Время кадра, что бы игра не зависила от fps
        velocity.x = 0.0;
        if is_key_down(KeyCode::A) {
            velocity.x -= 250.0;
        }
        if is_key_down(KeyCode::D) {
            velocity.x += 250.0;
        }
        if is_key_pressed(KeyCode::Space) {
            if on_ground {
                velocity.y = jump_force;
            } else if d_jump_count > 0 {
                velocity.y = jump_force;
                d_jump_count -= 1;
            }
        }
        let mouse_pos: Vec2 = mouse_position().into();
        let it_on = button_rect.contains(mouse_pos);
        if it_on && is_mouse_button_pressed(MouseButton::Left) {
            score += 1;
        }
        let old_pos = pos;
        velocity.y += gravity * dt; // гравитация
        pos += velocity * dt; // движение
        // Персонаж
        let player_rect = Rect::new(pos.x, pos.y, size.x, size.y);
        on_ground = false;
        // Коллизия
        for floor in &floors {
            if player_rect.overlaps(&floor) {
                if old_pos.y + size.y <= floor.y && velocity.y >= 0.0 {
                    pos.y = floor.y - size.y;
                    velocity.y = 0.0;
                    on_ground = true;
                    d_jump_count = 1;
                } else {
                    if old_pos.x + size.x / 2.0 >= floor.x + floor.w / 2.0 {
                        pos.x = floor.x + floor.w;
                    } else {
                        pos.x = floor.x - size.x;
                    }
                }
            }
        }

        // Отрисовка
        clear_background(Color::from_rgba(25, 25, 30, 255)); // r g b and trancperensy
        draw_rectangle(pos.x, pos.y, size.x, size.y, SKYBLUE);
        //draw_rectangle(platform.x, platform.y, platform.w, platform.h, BROWN);
        for floor in &floors {
            draw_rectangle(floor.x, floor.y, floor.w, floor.h, BROWN);
        }
        draw_rectangle(
            button_rect.x,
            button_rect.y,
            button_rect.w,
            button_rect.h,
            RED,
        );
        draw_text(&format!("Score: {score}"), 50.0, 50.0, 32.0, WHITE);
        next_frame().await
    }
}
