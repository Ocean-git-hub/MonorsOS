use alloc::vec::Vec;
use core::ffi::c_void;

use uefi_wrapper::{Event, print, println};
use uefi_wrapper::boot_services::{EventType, TPL};
use uefi_wrapper::protocols::console::text_output::Geometry;
use uefi_wrapper::time::TimerDelay;

use crate::*;

pub struct BootMenuOption {
    name: &'static str,
    action: fn(),
}

impl BootMenuOption {
    pub fn new(name: &'static str, action: fn()) -> BootMenuOption {
        Self {
            name,
            action,
        }
    }

    fn action(&self) {
        (self.action)();
    }
}

enum ReceiveFrom {
    Keyboard,
    Timeout,
}

pub struct BootMenu {
    options: Vec<BootMenuOption>,
    count_down_cursor: Geometry,
    press_key_cursor: Geometry,
    automatic_boot_start_cursor: Geometry,
    automatic_boot_end_cursor: Geometry,
    count: i32,
    timeout_event: Option<Event>,
    timer_event: Option<Event>,
}

impl BootMenu {
    pub fn new(auto_boot_seconds: i32, options: Vec<BootMenuOption>) -> Self {
        assert!(auto_boot_seconds > 0);
        assert!(auto_boot_seconds < 10);
        assert!(options.len() > 0);
        BootMenu {
            options,
            count_down_cursor: Geometry::default(),
            press_key_cursor: Geometry::default(),
            automatic_boot_start_cursor: Geometry::default(),
            automatic_boot_end_cursor: Geometry::default(),
            count: auto_boot_seconds,
            timeout_event: None,
            timer_event: None,
        }
    }

    pub fn menu_loop(&mut self) {
        self.print_options();
        println!();
        self.print_press_key();
        println!("\n");
        self.print_automatic_boot();

        self.set_count_down();

        let mut select_option = 1;
        match self.wait_event() {
            ReceiveFrom::Keyboard => {
                self.clear_timer();
                BootMenu::clear_characters(self.automatic_boot_start_cursor, self.automatic_boot_end_cursor);

                let current_cursor = BootMenu::current_cursor();
                loop {
                    let input_char = BootMenu::wait_for_input_char() as u8 as char;
                    if input_char >= '1'
                        && input_char <= ('0' as usize + self.options.len()) as u8 as char {
                        BootMenu::set_cursor(self.press_key_cursor);
                        select_option = input_char as usize - '0' as usize;
                        print!("{}", input_char);
                    } else if input_char == '\r' || input_char == '\n' {
                        break;
                    }
                }
                BootMenu::set_cursor(current_cursor);
            }
            ReceiveFrom::Timeout => {}
        }
        self.options[select_option - 1].action();
    }

    fn print_options(&self) {
        println!("Select option: ");
        for (i, option) in self.options.iter().enumerate() {
            println!("     {}: {}", i + 1, option.name);
        }
    }

    fn print_press_key(&mut self) {
        print!("Press key: [ ");
        self.press_key_cursor = BootMenu::current_cursor();
        println!("  ]");
    }

    fn print_automatic_boot(&mut self) {
        self.automatic_boot_start_cursor = BootMenu::current_cursor();
        print!("Automatic boot in ");
        self.count_down_cursor = BootMenu::current_cursor();
        println!("{} seconds...", self.count);
        self.automatic_boot_end_cursor = BootMenu::current_cursor();
    }

    fn clear_characters(start_cursor: Geometry, end_cursor: Geometry) {
        assert!(start_cursor.row() < end_cursor.row());
        let screen = con_out().query_mode(
            con_out().mode().mode_number() as usize
        ).unwrap();

        let current_cursor = BootMenu::current_cursor();
        BootMenu::set_cursor(start_cursor);
        let repeat = end_cursor.column() - start_cursor.column()
            + (end_cursor.row() - start_cursor.row()) * screen.column();
        for _ in 0..repeat {
            print!(" ");
        }
        BootMenu::set_cursor(current_cursor);
    }

    fn set_count_down(&mut self) {
        unsafe {
            self.timer_event = Some(
                boot_services().create_event(
                    EventType::TIMER | EventType::NOTIFY_SIGNAL,
                    TPL::CALLBACK,
                    Some(BootMenu::count_down_boot_timer),
                    Some(self as *mut BootMenu as *mut c_void),
                ).expect("Could not creat event")
            );
        }
        boot_services().set_timer(
            self.timer_event.unwrap(),
            TimerDelay::Periodic,
            10000000,
        ).expect("Could not set timer");
    }

    fn clear_timer(&mut self) {
        boot_services().set_timer(
            self.timer_event.unwrap(),
            TimerDelay::Cancel,
            0,
        ).expect("Could not cancel timer");
        boot_services().close_event(self.timer_event.unwrap());
        self.timer_event = None;
    }

    unsafe extern "efiapi" fn count_down_boot_timer(_event: Event, context: *mut c_void) {
        let mut boot_menu = (context as *mut BootMenu).as_mut()
            .expect("Could not access BootMenu");
        boot_menu.count -= 1;

        let current_cursor = BootMenu::current_cursor();
        BootMenu::set_cursor(boot_menu.count_down_cursor);
        print!("{}", boot_menu.count);

        BootMenu::set_cursor(current_cursor);

        if boot_menu.count == 0 {
            boot_menu.clear_timer();
            boot_menu.timeout_automatic_boot();
        }
    }

    fn create_timeout_event(&self) -> Event {
        unsafe {
            boot_services().create_event(
                EventType(0),
                TPL::CALLBACK,
                None,
                None,
            )
        }.expect("Could not creat event")
    }

    fn timeout_automatic_boot(&self) {
        boot_services().signal_event(
            self.timeout_event.expect("timeout_event is not set")
        ).expect("Could not signal event");
    }

    fn wait_event(&mut self) -> ReceiveFrom {
        self.timeout_event = Some(self.create_timeout_event());
        let index = boot_services().wait_for_event(
            &[
                con_in().wait_for_key(),
                self.timeout_event.unwrap()
            ]
        ).expect("Could not set wait_for_event");

        boot_services().close_event(self.timeout_event.unwrap());
        self.timeout_event = None;

        match index {
            0 => ReceiveFrom::Keyboard,
            1 => ReceiveFrom::Timeout,
            _ => ReceiveFrom::Keyboard
        }
    }

    fn wait_for_input_char() -> u16 {
        match con_in().input_read_key() {
            Ok(key) => key.unicode_char(),
            Err(_) => {
                boot_services().wait_for_event(&[con_in().wait_for_key()])
                    .expect("Could not set wait_for_event");
                con_in().input_read_key()
                    .expect("Could not read input key")
                    .unicode_char()
            }
        }
    }

    fn current_cursor() -> Geometry {
        con_out().mode()
            .cursor()
    }

    fn set_cursor(cursor: Geometry) {
        con_out().set_cursor_position(cursor)
            .expect("Could not set cursor");
    }
}
