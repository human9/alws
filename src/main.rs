extern crate alws;
use alws::*;

extern crate ncurses;
use ncurses::*;

fn main() {

    initscr();
    start_color();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    init_pair(1, COLOR_RED, COLOR_BLACK);
    
    let path = default_path();
    let file = open_file(&path);
    let log = open_log(&file);

    let mut missions = log.mission_list();
    let mut items = Vec::new();
    let mut i = 0;
    for mission in &missions {
        items.push(new_item(mission.title.clone(), mission.description.clone()));
        i += 1;
    }
    let my_menu = new_menu(&mut items);
    menu_opts_off(my_menu, O_SHOWDESC);

    let my_menu_win = newwin(9, 18, 4, 4);
    keypad(my_menu_win, true);
    
    set_menu_win(my_menu, my_menu_win);
    set_menu_sub(my_menu, derwin(my_menu_win, 5, 0, 2, 2));

    set_menu_mark(my_menu, " * ");

    box_(my_menu_win, 0, 0);
    mvprintw(LINES() - 3, 0, "Press <ENTER> to see the option selected");
    mvprintw(LINES() - 2, 0, "F1 to exit");
    refresh();

    /* Post the menu */
    post_menu(my_menu);
    wrefresh(my_menu_win);

    mv(20, 0);
    clrtoeol();
    
    let index = item_index(current_item(my_menu)) as usize;
    mvprintw(20, 0, &format!("Mission began: {}", missions[index].timestamp)[..]);
    mvprintw(21, 0, &format!("Mission description: {}", item_description(current_item(my_menu)))[..]);
    pos_menu_cursor(my_menu);
    let mut ch = getch();
    while ch != KEY_F(1) {
        match ch {
            KEY_UP => {
                menu_driver(my_menu, REQ_UP_ITEM);
            },
            KEY_DOWN => {
                menu_driver(my_menu, REQ_DOWN_ITEM);
            },
            10 => {/* Enter */
                mv(20, 0);
                clrtoeol();
                
                let index = item_index(current_item(my_menu)) as usize;
                mvprintw(20, 0, &format!("Mission began: {}", missions[index].timestamp)[..]);
                mvprintw(21, 0, &format!("Mission description: {}", item_description(current_item(my_menu)))[..]);
                pos_menu_cursor(my_menu);
            },
            _ => {}
        }
        wrefresh(my_menu_win);
        ch = getch();
    }
    unpost_menu(my_menu);

    /* free items */
    for &item in items.iter() {
        free_item(item);
    }
    free_menu(my_menu);
    endwin();
    
    //write_to_file(&path, &log);
}

