extern crate alws;
use alws::*;

extern crate chrono;
use chrono::prelude::*;


extern crate ncurses;
use ncurses::*;

fn clrprint(y: i32, x: i32, string: &str) {
    mv(y, x);
    clrtoeol();
    mvprintw(y, x, string);
}

fn clrprintw(window: WINDOW, y: i32, x: i32, string: &str) {
    wmove(window, y, x);
    wclrtoeol(window);
    mvwprintw(window, y, x, string);
}

fn wprint(window: WINDOW, string: &str) {
    wclrtoeol(window);
    wprintw(window, string);
}


struct LogView {
    menu: MENU,
    items: Vec<ITEM>,
    menu_window: WINDOW,
    details: WINDOW,
    details_window: WINDOW,
    log: Log,
}

impl LogView {
    fn new(log: Log) -> Self {
        
        let mut lv = LogView {
            menu: new_menu(&mut Vec::new()),
            items: Vec::new(),
            menu_window: newwin(1, 1, 0, 0),
            details: newwin(2, 2, 0, 0),
            details_window: newwin(2, 2, 0, 0),
            log,
        };

        lv.free_menu();
        lv.build_menu(0);
        lv
    }

    fn resize(&mut self) {
        let index = item_index(current_item(self.menu)) as usize;
        unpost_menu(self.menu);
        self.free_menu();
        self.build_menu(index);
    }

    fn up(&mut self) {
        menu_driver(self.menu, REQ_UP_ITEM);
        wrefresh(self.menu_window);
        self.draw_window();
    }

    fn down(&mut self) {
        menu_driver(self.menu, REQ_DOWN_ITEM);
        wrefresh(self.menu_window);
        self.draw_window();
    }

    fn free_menu(&mut self) {
        for &item in self.items.iter() {
            free_item(item);
        }
        self.items.clear();
    }

    fn build_menu(&mut self, index: usize) {

        for mission in &self.log.mission_list() {
            self.items.push(new_item(mission.title.clone(), mission.description.clone()));
        }
        let my_menu = new_menu(&mut self.items);
        menu_opts_off(my_menu, O_SHOWDESC);

        set_current_item(my_menu, self.items[index]);
        set_menu_mark(my_menu, "> ");

        let (mut rows, mut cols) = (0, 0);
        scale_menu(my_menu, &mut rows, &mut cols);
        rows = LINES() - 2;
        cols += 4;

        let my_menu_win = newwin(rows, cols, 0, 0);
        set_menu_win(my_menu, my_menu_win);
        let subwindow = derwin(my_menu_win, rows-2, cols-2, 2, 2);
        set_menu_sub(my_menu, subwindow);
        keypad(my_menu_win, true);

        box_(my_menu_win, 0, 0);
        mvwprintw(my_menu_win, 0, 2, "MISSION LIST");
        refresh();
        
        post_menu(my_menu);
        wrefresh(my_menu_win);
        
        self.menu = my_menu;
        self.menu_window = my_menu_win;

        wresize(self.details, LINES()-2, COLS()-cols);
        mvwin(self.details, 0, cols);
        wresize(self.details_window, LINES()-4, (COLS()-cols)-3);
        mvwin(self.details_window, 1, cols+2);

        self.draw_window();

    }


    fn draw_window(&mut self) {
        //need a subwindow to prevent destroying border

        let index = item_index(current_item(self.menu)) as usize;
        let ref mission = self.log.mission_list()[index];

        werase(self.details);
        box_(self.details, 0, 0);
        mvwprintw(self.details, 0, 2, "MISSION DETAILS");
        wrefresh(self.details);

        werase(self.details_window);

        wmove(self.details_window, 1, 0);

        let pretty_format = |ref utc: DateTime<Utc>| {
            let local = utc.with_timezone(&Local);
            let fmt = format!("%A, the {}{} of %B at %T", local.day(), day_suffixer(local.day()));
            local.format(&fmt).to_string()
        };

        let status = match &mission.completion {
            &None => "Ongoing".to_string(),
            &Some(ref dt) => format!("Completion on {}", pretty_format(dt.timestamp)),
        };
        wprint(self.details_window, &format!("{}\nStatus: {} since {}\n\nMission brief:\n{}\n", mission.title, status, pretty_format(mission.timestamp), mission.description));
        
        let basic_format = |ref utc: DateTime<Utc>| {
            let local = utc.with_timezone(&Local);
            let fmt = format!("%F at %T");
            local.format(&fmt).to_string()
        };

        for entry in &mission.entries {
            wprint(self.details_window, &format!("\n{}\n", basic_format(entry.timestamp)));
            wprint(self.details_window, &format!("{}\n", entry.entry_text));
        }

        wrefresh(self.details_window);

        // Probably needs to be redone anytime the details window is redone
        clrprint(LINES()-2, 0, "ALWS pre-alpha development build");
        clrprint(LINES()-1, 0, "Press <ENTER> to perform <UNSPECIFIED>, Q to quit");
    }

}


fn day_suffixer(day: u32) -> String {
    if day >= 11 && day <= 13 {
        return "th".to_string();
    }
    match day % 10 {
        1 => return "st".to_string(),
        2 => return "nd".to_string(),
        3 => return "rd".to_string(),
        _ => return "th".to_string(),

    }
}

fn main() {

    initscr();
    use_default_colors();
    start_color();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

    init_pair(1, -1, -1);
    
    let path = default_path();
    let file = open_file(&path);
    let mut lv = LogView::new(open_log(&file));

    let mut ch = getch();
    while ch != 81 && ch != 113 /* Upper and lower case Q */ {
        match ch {
            65 | 97 => {

            },
            KEY_RESIZE => {
                lv.resize();
            },
            KEY_UP => {
                lv.up();
            },
            KEY_DOWN => {
                lv.down();
            },
            10 => {/* Enter */
                pos_menu_cursor(lv.menu);
            },
            _ => {}
        }
        ch = getch();
    }
    
    endwin();

    write_to_file(&path, &lv.log);
}
