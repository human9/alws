extern crate alws;
use alws::*;

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

fn activate_fields(mission: &mut Mission, window: &mut WINDOW) {
    //need a subwindow to prevent destroying border
    mvwprintw(*window, 2, 2, &format!("TIMESTAMP: {}", mission.timestamp));
    mvwprintw(*window, 3, 2, &format!("MISSION BRIEF: {}", mission.description));
    wrefresh(*window);
}

//need to save menu position
fn show_menu(log: &mut Log, window: &mut WINDOW) -> bool {

    let mut missions = log.mission_list();

    let mut items = Vec::new();
    for mission in &missions {
        items.push(new_item(mission.title.clone(), mission.description.clone()));
    }
    let my_menu = new_menu(&mut items);
    menu_opts_off(my_menu, O_SHOWDESC);

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
    
    wresize(*window, LINES() - 2, COLS()-cols);
    mvwin(*window, 0, cols);
    wclear(*window);
    box_(*window, 0, 0);
    mvwprintw(*window, 0, 2, "MISSION DETAILS");
    let index = item_index(current_item(my_menu)) as usize;
    activate_fields(&mut missions[index], window);
    wrefresh(*window);

    post_menu(my_menu);
    wrefresh(my_menu_win);
    
    let free_menus = |items: &Vec<ITEM>| {
        for &item in items.iter() {
            free_item(item);
        }
    };
    
    clrprint(LINES() - 2, 0, "A to add new mission");
    clrprint(LINES() - 1, 0, "Press <ENTER> to see the option selected, Q to exit");

    let mut redraw = false;
    let mut ch = getch();
    while ch != 81 && ch != 113 {
        match ch {
            65 | 97 => {

            },
            KEY_RESIZE => {
                redraw = true;
                break;
            },
            KEY_UP => {
                menu_driver(my_menu, REQ_UP_ITEM);
            },
            KEY_DOWN => {
                menu_driver(my_menu, REQ_DOWN_ITEM);
            },
            10 => {/* Enter */
                pos_menu_cursor(my_menu);
                let index = item_index(current_item(my_menu)) as usize;
                activate_fields(&mut missions[index], window);
            },
            _ => {}
        }
        wrefresh(my_menu_win);
        ch = getch();
    }
    unpost_menu(my_menu);

    free_menus(&items);

    redraw
    
}

fn main() {

    initscr();
    start_color();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    init_pair(1, COLOR_RED, COLOR_BLACK);
    
    let path = default_path();
    let file = open_file(&path);
    let mut log = open_log(&file);

    let mut window = newwin(2, 2, 0, 0);
    while show_menu(&mut log, &mut window) == true {
    
    }
    
    endwin();

    write_to_file(&path, &log);
}
