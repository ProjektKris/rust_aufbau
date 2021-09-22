use iced::{
    button, text_input, Align, Button, Column, Element, Sandbox, Settings, Text, TextInput,
};

pub fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Orbital {
    S,
    P,
    D,
    F,
}

fn get_max_electron(orbital: Orbital) -> u32 {
    return match orbital {
        Orbital::S => 2,
        Orbital::P => 6,
        Orbital::D => 10,
        Orbital::F => 14,
    };
}

const ORBITAL_FILL_ORDER: [Orbital; 19] = [
    Orbital::S,
    Orbital::S,
    Orbital::P,
    Orbital::S,
    Orbital::P,
    Orbital::S,
    Orbital::D,
    Orbital::P,
    Orbital::S,
    Orbital::D,
    Orbital::P,
    Orbital::S,
    Orbital::F,
    Orbital::D,
    Orbital::P,
    Orbital::S,
    Orbital::F,
    Orbital::D,
    Orbital::P,
];

struct CurrentLayers {
    s: u32,
    p: u32,
    d: u32,
    f: u32,
}

#[derive(Default)]
struct Calculator {
    result: String,
    input: String,
    input_box: text_input::State,
    submit_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    InputSubmitted(String),
    SubmitPressed,
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Aufbau Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputSubmitted(input) => self.input = input,
            Message::SubmitPressed => {
                let electron = match self.input.parse::<u32>() {
                    Ok(res) => res,
                    Err(_) => 0,
                }; // will have to deal with error handling later :skull:
                let mut remaining_electrons = electron;
                let mut result = format!("electrons: {}\n", electron);
                let mut current_layers = CurrentLayers {
                    s: 0,
                    p: 1,
                    d: 2,
                    f: 3,
                };

                for i in 0..ORBITAL_FILL_ORDER.len() {
                    let orbital = ORBITAL_FILL_ORDER[i];
                    let max_electron = get_max_electron(orbital);

                    match orbital {
                        Orbital::S => current_layers.s += 1,
                        Orbital::P => current_layers.p += 1,
                        Orbital::D => current_layers.d += 1,
                        Orbital::F => current_layers.f += 1,
                    }

                    let layer = match orbital {
                        Orbital::S => current_layers.s,
                        Orbital::P => current_layers.p,
                        Orbital::D => current_layers.d,
                        Orbital::F => current_layers.f,
                    };
                    let orbital_str = match orbital {
                        Orbital::S => "s",
                        Orbital::P => "p",
                        Orbital::D => "d",
                        Orbital::F => "f",
                    };
                    if max_electron <= remaining_electrons {
                        remaining_electrons -= max_electron;
                        result = format!("{}{}{}{} ", result, layer, orbital_str, max_electron);
                    } else if remaining_electrons > 0 {
                        result =
                            format!("{}{}{}{} ", result, layer, orbital_str, remaining_electrons);
                        remaining_electrons = 0;
                    }
                }

                self.result = result;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(TextInput::new(
                &mut self.input_box,
                "1",
                &self.input,
                Message::InputSubmitted,
            ))
            .push(Text::new(String::from(self.result.clone())).size(20))
            .push(
                Button::new(&mut self.submit_button, Text::new("Submit"))
                    .on_press(Message::SubmitPressed),
            )
            .into()
    }
}
