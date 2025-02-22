use crate::container::Container;

mod container;

fn main() {
    let container = Container::new();

    unsafe {
        let _ = container.run();
    }
}
