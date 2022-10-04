pub struct Transform([f32; 16]);

impl Transform {
    pub fn new(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        g: f32,
        h: f32,
        i: f32,
        j: f32,
        k: f32,
        l: f32,
        m: f32,
        n: f32,
        o: f32,
        p: f32,
    ) -> Transform {
        Transform([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.0[x + y * 4]
    }
}
