// importing utilities from rdev
use rdev::{listen, Event, EventType};

fn main() {
    // declaring variables to store input
    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;
    let mut click_x: f64 = 0.0;
    let mut click_y: f64 = 0.0;
    let mut button = String::new();
    let mut keypress: Vec<String> = vec![];

    // the callback which is passed in listen methodd
    let callback = move |event: Event| {
        // Command::new("clear").status();

        // clears the terminal
        println!("\x1B[2J\x1B[1;1H");

        // prints the original captured event
        // println!("{:?}", event);

        // match type of event with the following and store the data of the event
        match event.event_type {
            // if event was mouse move
            EventType::MouseMove { x, y } => {
                // then store the coordinates of mouse
                mouse_x = x;
                mouse_y = y;
            }
            // if event was mouse click
            EventType::ButtonPress(btn) => {
                // save the button which it was
                button = format!("{:?}", btn);
                // and the co-ordinates of the click
                click_x = mouse_x;
                click_y = mouse_y;
            }
            // if event was keypress
            EventType::KeyPress(key) => {
                // match for each type of keys
                match key {
                    _ => {
                        // format the key type to only get the key entered
                        let value = format!("{:?}", key).as_str().replace("Key", "");
                        // if the key is not counted already
                        if !keypress.contains(&value) {
                            // then count it in
                            keypress.push(value);
                        }
                    }
                }
            }
            // on key release
            EventType::KeyRelease(key) =>
            // match for each type of key
             match key {
                _ => {
                    // again format to just get the key entered
                    let value = format!("{:?}", key).as_str().replace("Key", "");
                    // if the key is counted which should be the case 
                    if keypress.contains(&value) {
                        // uncount the key
                        let index = index_of(&keypress, value);
                        if index >= 0 {
                            keypress.remove(index as usize);
                        }
                    }
                }
            },
            _ => {}
        }
        // log the data
        println!("Mouse: {mouse_x}, {mouse_y} \nClick: {click_x}, {click_y} \nButton: {button} \nKeypress: {}", formated(&keypress));
    };

    // listens for input events and if any error occured then print the error
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

// this method returns the index of given value in given vector
pub fn index_of(vector: &Vec<String>, value: String) -> i32 {
    // turns vector into a interator and compares each item in the vector to the given value if and
    // of the item matched then the index of that item is stored in index and then the index is
    // returned else it comes out of if statement and returns -1 which indicates the value was not
    // found in vector
    if let Some(index) = vector.iter().position(|item| item == &value) {
        return index as i32;
    }
    -1
}

// format the given keypress vector for print statement
pub fn formated(vector: &Vec<String>) -> String {
    let mut formated = String::new();
    for element in vector {
        if vector.get(0) != Some(element) {
            formated = formated + " + " + element;
        } else {
            formated = formated + element;
        }
    }

    formated
}
