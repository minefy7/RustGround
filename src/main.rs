use macroquad::prelude::*;

struct Blocks {
    rect: Rect,
    color: Color,
}

#[macroquad::main("RustGround")]
async fn main() {
    // Параметры персонажа
    let mut pos = vec2(200.0, 300.0);
    let size = vec2(32.0, 32.0);
    let mut velocity = vec2(0.0, 0.0);
    let mut on_ground = false;

    // Параметры геймплея и инкрементации
    let mut score: i32 = 0;
    let mut base_click: i32 = 1;
    let mut cost_of_one_click: i32 = 20;
    // Параметры физики
    //let mut speed = 250.0;
    let gravity = 900.0;
    let jump_force = -420.0;

    // Пол (плейсхолдер), x, y, ширина, высота
    let blocks = vec![
        Blocks {
            rect: Rect::new(0.0, 400.0, 400.0, 200.0),
            color: BROWN,
        },
        Blocks {
            rect: Rect::new(600.0, 400.0, 400.0, 200.0),
            color: BROWN,
        },
        Blocks {
            rect: Rect::new(650.0, 250.0, 60.0, 70.0),
            color: GRAY,
        },
    ];

    //Кнопки
    let buttons = vec![Rect::new(150.0, 100.0, 100.0, 100.0)];
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
        let it_on = buttons[0].contains(mouse_pos);
        if it_on && is_mouse_button_pressed(MouseButton::Left) {
            score += base_click;
        }
        let old_pos = pos;
        velocity.y += gravity * dt; // гравитация
        pos += velocity * dt; // движение
        // Персонаж
        let player_rect = Rect::new(pos.x, pos.y, size.x, size.y);
        on_ground = false;

        // Коллизия
        let mut hit_upgrade = false;
        for (i, block) in blocks.iter().enumerate() {
            if player_rect.overlaps(&block.rect) {
                if old_pos.y + size.y <= block.rect.y && velocity.y >= 0.0 {
                    pos.y = block.rect.y - size.y;
                    velocity.y = 0.0;
                    on_ground = true;
                    d_jump_count = 1;
                } else if old_pos.y >= block.rect.y + block.rect.h {
                    pos.y = block.rect.y + block.rect.h;
                    velocity.y = 0.0;
                    if i == 2 {
                        hit_upgrade = true;
                    }
                } else {
                    if old_pos.x + size.x / 2.0 >= block.rect.x + block.rect.w / 2.0 {
                        pos.x = block.rect.x + block.rect.w;
                    } else {
                        pos.x = block.rect.x - size.x;
                    }
                }
            }
        }
        if hit_upgrade && score >= cost_of_one_click {
            base_click += 1;
            score -= cost_of_one_click;
            cost_of_one_click = cost_of_one_click * 3 / 2;
        }
        // Отрисовка
        clear_background(Color::from_rgba(25, 25, 30, 255)); // r g b and trancperensy
        draw_rectangle(pos.x, pos.y, size.x, size.y, SKYBLUE);
        //draw_rectangle(platform.x, platform.y, platform.w, platform.h, BROWN);
        for block in &blocks {
            draw_rectangle(
                block.rect.x,
                block.rect.y,
                block.rect.w,
                block.rect.h,
                block.color,
            );
        }
        for button in &buttons {
            draw_rectangle(button.x, button.y, button.w, button.h, RED);
        }
        draw_text(&format!("Score: {score}"), 50.0, 50.0, 32.0, WHITE);
        draw_text(
            &format!("Cost: {cost_of_one_click}"),
            650.0,
            200.0,
            20.0,
            WHITE,
        );
        next_frame().await
    }
}
