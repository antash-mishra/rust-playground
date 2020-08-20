enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x:i64, y:i64 },
}

fn inspect (event:WebEvent ) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("key pressed '{}' ", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y} => {
            println!("Clicked at x={}, y={}.", x, y);
        
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned()); // to_owned() creates an owned string from string slice
    let click = WebEvent::Click{x:20, y:80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

}