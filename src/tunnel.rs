use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor, style::Print, terminal,
};
use std::io::{stdout, Write};
use std::{thread, time::Duration};
use std::f32::consts::PI;
use crate::vmath;
use crate::vmath::Vector2;

fn draw_frame(buffer: &mut String, width: usize, height: usize, time: f64) {
    buffer.clear();
    static GRADIENT: [char; 10] = [' ', '.', ',', ';', 'p', '0', '8', '&', '#', '@']; // @#&80p;,.
    for y in 0..height {
        for x in 0..width {
            let mut i = x as f32;
            let mut j = y as f32;
            i /= width as f32;
            j /= height as f32;
            i = i * (width as f32/height as f32);
            i = i * (11f32/22f32);
            i = (i - 0.5) * 2f32;
            j = (j - 0.5) * 2f32;
            let rho = 0.25f32/vmath::v_dist(Vector2{x: i as f64, y: j as f64}, Vector2{x: 0f64, y: 0f64}) as f32;
            let pho = libm::atan((i / j) as f64) as f32 / PI;
            let d = vmath::v_dist(Vector2{x: i as f64, y: j as f64}, Vector2{x: 0f64, y: 0f64});
            let w = GRADIENT[libm::fmax(libm::fmin(d*2f64*9f64, 9f64), 0f64) as usize];
            buffer.push(if (rho+(time as f32 / 2f32)) % 0.3 < 0.15 && pho % 0.15 < 0.15/2.0 {w} else {' '});
        }
        buffer.push('\n');
    }
}

pub(crate) fn do_work() {
    let mut stdout = stdout();
    let (width, height) = (80*2, 24*2); // Размеры окна
    let mut buffer = String::with_capacity(width * height);

    // Инициализация терминала
    stdout.execute(terminal::EnterAlternateScreen).expect("Error at exec");
    terminal::enable_raw_mode().expect("Error at exec");

    for i in 0.. {
        let time = i as f64 * 0.01;
        draw_frame(&mut buffer, width, height, time);

        // Очистка и обновление экрана
        stdout.queue(cursor::Hide).expect("Error at exec")
              .queue(cursor::MoveTo(0, 0)).expect("Error at exec")
              .queue(Print(&buffer)).expect("Error at exec")
              .flush().expect("Error at exec");

        // Установка задержки для плавной анимации
        thread::sleep(Duration::from_millis(10));
    }

    // Возвращение в обычный режим перед выходом
    stdout.execute(terminal::LeaveAlternateScreen).expect("Error at exec");
    terminal::disable_raw_mode().expect("Error at exec");
    stdout.execute(cursor::Show).expect("Error at exec");

}
