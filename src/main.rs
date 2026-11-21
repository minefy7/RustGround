use macroquad::prelude::*;
// Camera2D — встроенная 2D-камера macroquad.
// set_camera включает её, set_default_camera выключает (вернуть обычные экранные координаты).
use macroquad::camera::{Camera2D, set_camera, set_default_camera};

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
    let mut cash: i32 = 0;
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
            rect: Rect::new(600.0, 400.0, 600.0, 200.0),
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
            cash += base_click;
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
        if hit_upgrade && cash >= cost_of_one_click {
            base_click += 1;
            cash -= cost_of_one_click;
            cost_of_one_click = cost_of_one_click * 3 / 2;
        }
        // Отрисовка
        clear_background(Color::from_rgba(25, 25, 30, 255)); // r g b and trancperensy

        // ===== КАМЕРА =====
        // Идея: камера — это "окно", через которое мы смотрим на мир.
        // from_display_rect(rect) говорит: "вот этот прямоугольник МИРА
        // растяни на весь экран".
        // Мы каждый кадр строим этот прямоугольник так, чтобы его ЦЕНТР
        // совпадал с позицией персонажа:
        //   левый край   = pos.x - половина ширины экрана
        //   верхний край = pos.y - половина высоты экрана
        // Размер окна = размеру экрана, т.е. масштаб 1:1, без зума.
        let camera = Camera2D::from_display_rect(Rect::new(
            pos.x - screen_width() / 2.0,
            pos.y + screen_height() / 2.0,
            screen_width(),
            -screen_height(),
        ));
        set_camera(&camera); // ВКЛЮЧИЛИ камеру: всё, что рисуем дальше,
        // сдвигается/масштабируется относительно неё.

        // --- МИР (рисуется ПОД камерой, т.е. двигается вместе с ней) ---
        draw_rectangle(pos.x, pos.y, size.x, size.y, SKYBLUE);
        for block in &blocks {
            draw_rectangle(
                block.rect.x,
                block.rect.y,
                block.rect.w,
                block.rect.h,
                block.color,
            );
        }

        // Цена прокачки — часть МИРА: рисуем ПОД камерой, прямо над
        // серым блоком (blocks[2]), чтобы текст "ездил" вместе с блоком.
        // Координаты — мировые: левый край блока, на 10 пикселей выше его верха.
        let shop = &blocks[2];
        draw_text(
            &format!("Cost: {cost_of_one_click}"),
            shop.rect.x,
            shop.rect.y - 10.0,
            20.0,
            WHITE,
        );

        set_default_camera(); // ВЫКЛЮЧИЛИ камеру: вернули обычные
        // экранные координаты (0,0 — левый верхний угол окна).

        // --- UI (рисуется БЕЗ камеры, прибит к экрану) ---
        // Если бы кнопку и текст рисовать до set_default_camera(),
        // они бы "уезжали" вместе с миром.
        for button in &buttons {
            draw_rectangle(button.x, button.y, button.w, button.h, RED);
        }
        draw_text(&format!("Cash: {cash}"), 50.0, 50.0, 32.0, WHITE);
        next_frame().await
    }
}
