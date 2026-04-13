use std::time::{Instant, Duration};
use std::thread::sleep;
use std::io::{stdout, Write, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crossterm::{
    cursor,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    style::{Color, SetForegroundColor, ResetColor},
    ExecutableCommand,
};
use rand;

struct Source {
    x_0: f32,
    y_0: f32,
    t_start: Instant,
}

fn calculate_wave_intensity(x: u16, y: u16, source: &Source) -> f32 {
    let t = source.t_start.elapsed().as_secs_f32();
    
    let dx = (x as f32 - source.x_0) * 0.5; 
    let dy = y as f32 - source.y_0;
    let d = (dx * dx + dy * dy).sqrt();

    let v = 15.0;      
    let k = 2.0;       
    let gamma = 1.67;   
    let max_radius = 16.0; 

    if d > max_radius || d > v * t {
        return 0.0;
    }

    let phase = k * d - v * t;
    let amplitude = (-(d * 0.2 + t * gamma)).exp(); 
    let wave = (phase.sin()).max(0.0);

    wave * amplitude
}

fn main() -> Result<()> {
    
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Fehler beim Setzen des Strg+C Handlers");

    
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?; 
    stdout.execute(cursor::Hide)?;          
    stdout.execute(SetForegroundColor(Color::Blue))?; 

    let mut sources: Vec<Source> = Vec::new();
    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let fps_duration = Duration::from_millis(30);

    
    while running.load(Ordering::SeqCst) {
        let (cols, rows) = terminal::size()?;

        if rand::random::<f32>() < 0.1 {
            sources.push(Source {
                x_0: (rand::random::<f32>() * cols as f32),
                y_0: (rand::random::<f32>() * rows as f32),
                t_start: Instant::now(),
            });
        }

        sources.retain(|s| s.t_start.elapsed().as_secs_f32() < 2.0);

        stdout.execute(cursor::MoveTo(0, 0))?;

        let mut output = String::with_capacity((cols * rows) as usize);

        for y in 0..rows {
            for x in 0..cols {
                let mut total_intensity: f32 = 0.0;
                for source in &sources {
                    total_intensity += calculate_wave_intensity(x, y, &source);
                }

                total_intensity *= 5.0; 
                total_intensity = total_intensity.clamp(0.0, 1.0);
                
                let char_idx = (total_intensity * (chars.len() - 1) as f32) as usize;
                output.push(chars[char_idx]);
            }

            if y < rows - 1 { output.push('\n'); }
        }

        write!(stdout, "{}", output)?;
        stdout.flush()?;
        sleep(fps_duration);
    }


    stdout.execute(ResetColor)?;
    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}