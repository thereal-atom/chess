pub struct Component {
    name: String,
    x: f32,
    y: f32,
}

pub extern fn create_component(name: String) -> Component {
    Component {
        name,
        x: 0.0,
        y: 0.0,
    }
}