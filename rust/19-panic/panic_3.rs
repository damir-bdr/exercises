enum DoorState {
    Opened,
    Closed,
}

enum DoorAction {
    Open,
    Close,
}

fn take_action(current_state: DoorState, action: DoorAction) {
    match (current_state, action) {
        (DoorState::Opened, DoorAction::Close) => {
            unimplemented!()
        },
        (DoorState::Closed, DoorAction::Open) => {
        },
        _ => unreachable!(),
    }
}

fn main() {
    take_action(DoorState::Opened, DoorAction::Close);
    // take_action(DoorState::Opened, DoorAction::Open);
}