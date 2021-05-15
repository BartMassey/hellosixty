use std::cell::RefCell;
use std::rc::Rc;

use sixtyfps;

sixtyfps::sixtyfps! {
    import { Button } from "sixtyfps_widgets.60";

    export HelloGui := Window {
        callback increment();
        callback quit();
        property <int> counter: 0;
        width: 200px;
        height: 200px;
        default-font-family: "sans";
        default-font-size: 24pt;
        
        GridLayout {
            Row {
                Text {
                    text: root.counter;
                }
            }
            Row {
                Button {
                    text: "Increment";
                    clicked => { root.increment(); }
                }
                Button {
                    text: "Quit";
                    clicked => { root.quit(); }
                }
            }
        }
    }
}

fn main() {
    let app = Rc::new(RefCell::new(HelloGui::new()));
    let count = Rc::new(RefCell::new(0i32));
    let icount = Rc::clone(&count);
    let iapp = Rc::clone(&app);
    let mapp = app.borrow();
    mapp.on_increment(move || {
        let mut c = icount.borrow_mut();
        *c += 1;
        iapp.borrow().set_counter(*c);
    });
    mapp.on_quit(move || {
        println!("count {}", *count.borrow());
        std::process::exit(0);
    });
    mapp.run();
}
