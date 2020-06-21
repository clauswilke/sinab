use super::renderer::*;
use std::str;

fn string_manip(input: &str) {
    for line in input.lines() {
        for word in line.split(" ") {
            println!("--{}--", word)
        }
        println!("<newline>")
    }
}


fn make_grobs(rdev: &mut RenderDevice) {
    let gc = GContext::new();
    let mut m = rdev.string_metrics(" ", &gc);
    let w_space = m.width;
    let mut x = 0.2;
    let y = 0.5;
    rdev.draw_text("These", x, y, &gc);
    m = rdev.string_metrics("These", &gc);
    x += w_space + m.width;
    let mut gc2 = gc.copy();
    gc2.set_color("red");
    gc2.set_fontface(Fontface::Bold);
    rdev.draw_text("grobs", x, y, &gc2);
    m = rdev.string_metrics("grobs", &gc2);
    x += w_space + m.width;
    rdev.draw_text("were made", x, y, &gc);
    m = rdev.string_metrics("were made", &gc);
    x += w_space + m.width;
    gc2.set_color("blue");
    gc2.set_fontface(Fontface::Italics);
    rdev.draw_text("in rust.", x, y, &gc2);
}


#[no_mangle]
pub extern "C" fn test_renderer(rdev_ptr: *mut CRenderDevice) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    make_grobs(&mut rdev);

    string_manip("This is a test.\n And some more.")
}


// keep an empty test here for now as a reminder to write proper unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
