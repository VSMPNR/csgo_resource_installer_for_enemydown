use std::future::Future;
use std::intrinsics::transmute;
use std::cell::RefCell;

pub struct Progress {
    pub running: bool,
    pub progress: f32,
    pub message: String,
}

impl Progress {
    async fn draw_task(cell: &std::cell::RefCell<&mut Self>) {
        let term = console::Term::buffered_stdout();
        let mut output = String::new();

        let mut prev_text_len = term.size().1 as usize;

        let start_time = std::time::SystemTime::now();

        let mut running = cell.borrow().running;

        while running {
            running = cell.borrow().running;
            let time_string = {
                let t = start_time.elapsed().unwrap().as_secs();
                format!("  {}:{:>02} ", t / 60, t % 60)
            };

            let percentage_string = {
                format!(" {: >3}% ", (cell.borrow().progress * 100.0) as i32)
            };

            let (_, width) = term.size();
            let bar_width = width as isize - (time_string.len() + percentage_string.len() + 2 + cell.borrow().message.len()) as isize;

            output.clear();
            output.push_str(&cell.borrow().message);
            output.push_str(&time_string);
            if bar_width > 0 {
                let done_len = (bar_width as f32 * cell.borrow().progress) as i32;
                output.push('[');

                for _ in 0..done_len {
                    output.push('#');
                }
    
                for _ in 0..(bar_width as i32 - done_len) {
                    output.push('-');
                }
    
                output.push(']');
            }
            output.push_str(&percentage_string);


            let text_width = prev_text_len / (width as usize + 1) + 1;

            prev_text_len = console::measure_text_width(&output);

            term.write_line(&output).unwrap();
            term.flush().unwrap();
            term.clear_to_end_of_screen().unwrap();
            term.move_cursor_up(text_width).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    pub fn new() -> Self {
        Self { running: true, progress: 0.0, message: String::new() }
    }

    async fn update_task<Fut, F>(action: F, cell: &'static RefCell<&mut Self>)
    where
        F: FnOnce(&'static RefCell<&mut Self>) -> Fut,
        Fut: Future<Output = ()>,
    {
        action(cell).await;
        cell.borrow_mut().running = false;
    }

    pub async fn draw<F, Fut>(&mut self, action: F)
    where
        F: FnOnce(&'static RefCell<&mut Self>) -> Fut,
        Fut: Future<Output = ()>,
    {
        let cell = RefCell::new(self);
        let cell_r: &'static RefCell<&mut Self> =  unsafe { transmute(&cell) };
        tokio::join!(Self::update_task(action, cell_r), Self::draw_task(&cell));
    }
}