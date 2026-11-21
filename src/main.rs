use macroquad::prelude::*;
// Camera2D — встроенная 2D-камера macroquad.
// set_camera включает её, set_default_camera выключает (вернуть обычные экранные координаты).
use macroquad::camera::{Camera2D, set_camera, set_default_camera};

struct Blocks {
    rect: Rect,
    color: Color,
    kind: BlockKind,
}

enum BlockKind {
    Solid,
    ShopClick,
    ShopAuto,
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
    let mut sum_of_click_upgrades: i32 = 0;
    let mut cost_autoclicker = 25;
    let mut autoclickers = 0;
    let mut auto_timer: f32 = 0.0;
    let auto_delay: f32 = 1.0;
    // Параметры физики
    //let mut speed = 250.0;
    let gravity = 900.0;
    let jump_force = -420.0;

    // Пол (плейсхолдер), x, y, ширина, высота
    let blocks = vec![
        Blocks {
            rect: Rect::new(-500.0, 400.0, 6000.0, 600.0),
            color: DARKGRAY,
            kind: BlockKind::Solid,
        },
        Blocks {
            rect: Rect::new(650.0, 250.0, 60.0, 70.0),
            color: GRAY,
            kind: BlockKind::ShopClick,
        },
        Blocks {
            rect: Rect::new(650.2, 310.0, 59.5, 10.0),
            color: GREEN,
            kind: BlockKind::Solid,
        },
        Blocks {
            rect: Rect::new(750.0, 250.0, 60.0, 70.0),
            color: GRAY,
            kind: BlockKind::ShopAuto,
        },
        Blocks {
            rect: Rect::new(750.2, 310.0, 59.5, 10.0),
            color: GREEN,
            kind: BlockKind::Solid,
        },
        Blocks {
            rect: Rect::new(-1000.0, -200.0, 900.0, 2000.0),
            color: DARKGRAY,
            kind: BlockKind::Solid,
        },
        Blocks {
            rect: Rect::new(1200.0, -200.0, 900.0, 2000.0),
            color: DARKGRAY,
            kind: BlockKind::Solid,
        },
        Blocks {
            rect: Rect::new(-1000.0, -650.0, 6000.0, 600.0),
            color: DARKGRAY,
            kind: BlockKind::Solid,
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
        let old_pos = pos;
        velocity.y += gravity * dt; // гравитация
        pos += velocity * dt; // движение
        // Персонаж
        let player_rect = Rect::new(pos.x, pos.y, size.x, size.y);
        on_ground = false;
        // Коллизия
        let mut hit_upgrade_click = false;
        let mut hit_upgrade_autoclick = false;
        for block in &blocks {
            if player_rect.overlaps(&block.rect) {
                if old_pos.y + size.y <= block.rect.y && velocity.y >= 0.0 {
                    pos.y = block.rect.y - size.y;
                    velocity.y = 0.0;
                    on_ground = true;
                    d_jump_count = 1;
                } else if old_pos.y >= block.rect.y + block.rect.h {
                    pos.y = block.rect.y + block.rect.h;
                    velocity.y = 0.0;
                    match block.kind {
                        BlockKind::ShopClick => hit_upgrade_click = true,
                        BlockKind::ShopAuto => hit_upgrade_autoclick = true,
                        BlockKind::Solid => {}
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
        if hit_upgrade_click && cash >= cost_of_one_click {
            base_click += 1;
            cash -= cost_of_one_click;
            sum_of_click_upgrades += 1;
            cost_of_one_click = cost_of_one_click * 3 / 2;
        }
        if hit_upgrade_autoclick && cash >= cost_autoclicker {
            cash -= cost_autoclicker;
            autoclickers += 1;
            cost_autoclicker = cost_autoclicker * 4 / 2;
        }
        if autoclickers >= 1 {
            auto_timer += get_frame_time();
            if auto_timer >= auto_delay {
                cash += autoclickers;
                auto_timer -= auto_delay;
            }
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
        let zoom = 2.0;
        let w = screen_width() / zoom;
        let h = screen_height() / zoom;
        let camera =
            Camera2D::from_display_rect(Rect::new(pos.x - w / 2.0, pos.y + h / 4.0, w, -h));
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
        let mouse_pos: Vec2 = mouse_position().into();
        let mouse_world = camera.screen_to_world(mouse_pos);
        let it_on = buttons[0].contains(mouse_world);
        if it_on && is_mouse_button_pressed(MouseButton::Left) {
            cash += base_click;
        }
        // Подписи магазинов: ищем блоки по виду (BlockKind), а не по индексу —
        // порядок в vec! больше ничего не ломает. find возвращает Option,
        // поэтому рисуем только если блок нашёлся (if let Some).
        // Координаты — мировые, текст прибит к блоку магазина.
        if let Some(shop) = blocks
            .iter()
            .find(|b| matches!(b.kind, BlockKind::ShopClick))
        {
            draw_text(
                "Clicks",
                shop.rect.x + 10.0,
                shop.rect.y - 30.0,
                16.0,
                WHITE,
            );
            draw_text(
                &format!("{sum_of_click_upgrades}"),
                shop.rect.x + 24.0,
                shop.rect.y - 20.0,
                16.0,
                WHITE,
            );
            draw_text(
                &format!("Cost: {cost_of_one_click}"),
                shop.rect.x,
                shop.rect.y - 10.0,
                20.0,
                WHITE,
            );
        }
        if let Some(shop) = blocks
            .iter()
            .find(|b| matches!(b.kind, BlockKind::ShopAuto))
        {
            draw_text(
                "AutoClickers",
                shop.rect.x + 5.0,
                shop.rect.y - 30.0,
                12.0,
                WHITE,
            );
            draw_text(
                &format!("{autoclickers}"),
                shop.rect.x + 24.0,
                shop.rect.y - 20.0,
                16.0,
                WHITE,
            );
            draw_text(
                &format!("Cost: {cost_autoclicker}"),
                shop.rect.x,
                shop.rect.y - 10.0,
                20.0,
                WHITE,
            );
        }
        for button in &buttons {
            draw_rectangle(button.x, button.y, button.w, button.h, RED);
        }
        set_default_camera(); // ВЫКЛЮЧИЛИ камеру: вернули обычные
        // экранные координаты (0,0 — левый верхний угол окна).

        // --- UI (рисуется БЕЗ камеры, прибит к экрану) ---
        // Если бы кнопку и текст рисовать до set_default_camera(),
        // они бы "уезжали" вместе с миром.
        draw_text(&format!("Cash: {cash}"), 900.0, 50.0, 32.0, WHITE);
        next_frame().await
    }
}
