pub struct Ease;

impl Ease {

    pub fn in_out(t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }

}