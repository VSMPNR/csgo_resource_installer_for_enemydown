use console::Key;

pub struct Menu<'a> {
    elements : Vec<&'a str>
}

impl<'a> Menu<'a> {
    pub fn new() -> Self {
        Menu { elements : Vec::new() }
    }

    pub fn add(&mut self, name : &'a str) {
        self.elements.push(name);
    }

    pub fn select(&self, mut select: isize) -> usize {
        let term = console::Term::buffered_stdout();//console::Term::stdout();
        let active_style = console::Style::new().on_white().black();

        let element_amount = self.elements.len() as isize;

        let mut tmp_string_1 = String::new();
        let mut tmp_string_2 = String::new();

        let mut draw_update = true;
        
        loop {
            let (_, term_width) = term.size();

            if draw_update {
                //term.clear_line().unwrap();
                tmp_string_2.clear();
    
                let mut count = 0;
                for elem in &self.elements {
                    tmp_string_2.push(' ');
                    tmp_string_1.clear();
    
                    tmp_string_1.push(' ');
                    tmp_string_1.push_str(elem);
                    tmp_string_1.push(' ');
    
                    if count == select {
                        tmp_string_2.push_str(&active_style.apply_to(&tmp_string_1).to_string());
                    } else {
                        tmp_string_2.push_str(&tmp_string_1);
                    }
                    count += 1;
                }
                tmp_string_2.push_str("\x1b[0m\x1b[0K");
                term.write_line(&tmp_string_2).unwrap();
                let len = console::measure_text_width(&tmp_string_2);
                term.flush().unwrap();
                term.move_cursor_up(len / (term_width as usize + 1) + 1).unwrap();
            }

            //if let Ok(key) = receive_key.try_recv() {}

            match term.read_key().unwrap() {
                Key::ArrowLeft  => {
                    select = (select - 1).max(0);
                    draw_update = true;
                },
                Key::ArrowRight => {
                    select = (select + 1).min(element_amount - 1);
                    draw_update = true;
                },
                Key::Enter => { break; },
                _ => {},
            };
        }

        term.clear_to_end_of_screen().unwrap();
        term.flush().unwrap();
        return select.max(0) as usize;
    }
}