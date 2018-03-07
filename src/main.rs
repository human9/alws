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

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    init_pair(1, COLOR_RED, COLOR_BLACK);
    
    let path = default_path();
    let file = open_file(&path);
    let mut log = open_log(&file);

    {
        let mut missions = log.mission_list();
        let mut items = Vec::new();
        for mission in &missions {
            items.push(new_item(mission.title.clone(), mission.description.clone()));
        }
        let my_menu = new_menu(&mut items);
        menu_opts_off(my_menu, O_SHOWDESC);

        let my_menu_win = newwin(12, max_x, 0, 0);
        keypad(my_menu_win, true);
        
        set_menu_win(my_menu, my_menu_win);
        set_menu_sub(my_menu, derwin(my_menu_win, 5, 0, 2, 2));

        set_menu_mark(my_menu, " > ");

        box_(my_menu_win, 0, 0);
        mvprintw(LINES() - 2, 0, "A to add new mission");
        mvprintw(LINES() - 1, 0, "Press <ENTER> to see the option selected, Q to exit");
        refresh();

        /* Post the menu */
        post_menu(my_menu);
        wrefresh(my_menu_win);

        mv(20, 0);
        clrtoeol();
            
        let repaint_menu = || {
            clear();
            wclear(my_menu_win);
            set_menu_win(my_menu, my_menu_win);
            set_menu_sub(my_menu, derwin(my_menu_win, 5, 0, 2, 2));
            set_menu_mark(my_menu, " > ");
            box_(my_menu_win, 0, 0);
            mvprintw(LINES() - 2, 0, "A to add new mission");
            mvprintw(LINES() - 1, 0, "Press <ENTER> to see the option selected, Q to exit");
            let index = item_index(current_item(my_menu)) as usize;
            mvprintw(13, 0, &format!("Mission began: {}", missions[index].timestamp)[..]);
            mvprintw(14, 0, &format!("Mission description: {}", item_description(current_item(my_menu)))[..]);
            refresh();
        };
        
        mvprintw(LINES() - 2, 0, "A to add new mission");
        mvprintw(LINES() - 1, 0, "Press <ENTER> to see the option selected, Q to exit");
        let index = item_index(current_item(my_menu)) as usize;
        mvprintw(13, 0, &format!("Mission began: {}", missions[index].timestamp)[..]);
        mvprintw(14, 0, &format!("Mission description: {}", item_description(current_item(my_menu)))[..]);

        pos_menu_cursor(my_menu);

        let mut ch = getch();
        while ch != 81 && ch != 113 {
            match ch {
                65 | 97 => {

                },
                KEY_RESIZE => {
                    repaint_menu();
                },
                KEY_UP => {
                    menu_driver(my_menu, REQ_UP_ITEM);
                },
                KEY_DOWN => {
                    menu_driver(my_menu, REQ_DOWN_ITEM);
                },
                10 => {/* Enter */
                    mv(20, 0);
                    clrtoeol();
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
    
    }

    write_to_file(&path, &log);
}

fn draw_menu() {

}

