use core::game::action::{Action, Create};

fn main() {
    let a: Action = core::game::action::Action::Create(Create { type_id: 4 });

    let mut aa: Vec<Action> = vec![];
    aa.push(a);
    let json_string = serde_json::to_string(&aa).unwrap();
    println!("{}", json_string);
}
