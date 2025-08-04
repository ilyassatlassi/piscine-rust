#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut vec_color: Vec<u8> = vec![self.r, self.g, self.b, self.a];
        let mut first_el = 0;
        let mut second_el = 0;

        for (i, _) in vec_color.iter().enumerate() {
            if vec_color[i] == first {
                first_el = i;
            } else if vec_color[i] == second {
                second_el = i;
            }
        }
        vec_color.swap(first_el, second_el);
        Self {
            r: vec_color[0],

            g: vec_color[1],

            b: vec_color[2],

            a: vec_color[3],
        }
    }
}
#[test]
fn test_one() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };
    // swap r
    assert_eq!(
        c.swap(c.r, c.b),
        Color {
            r: 10,
            g: 200,
            b: 255,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.r, c.g),
        Color {
            r: 200,
            g: 255,
            b: 10,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.r, c.a),
        Color {
            r: 30,
            g: 200,
            b: 10,
            a: 255
        }
    );

    // swap g
    assert_eq!(
        c.swap(c.g, c.r),
        Color {
            r: 200,
            g: 255,
            b: 10,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.g, c.b),
        Color {
            r: 255,
            g: 10,
            b: 200,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.g, c.a),
        Color {
            r: 255,
            g: 30,
            b: 10,
            a: 200
        }
    );

    // swap b
    assert_eq!(
        c.swap(c.b, c.r),
        Color {
            r: 10,
            g: 200,
            b: 255,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.b, c.g),
        Color {
            r: 255,
            g: 10,
            b: 200,
            a: 30
        }
    );
    assert_eq!(
        c.swap(c.b, c.a),
        Color {
            r: 255,
            g: 200,
            b: 30,
            a: 10
        }
    );

    // swap a
    assert_eq!(
        c.swap(c.a, c.r),
        Color {
            r: 30,
            g: 200,
            b: 10,
            a: 255
        }
    );
    assert_eq!(
        c.swap(c.a, c.b),
        Color {
            r: 255,
            g: 200,
            b: 30,
            a: 10
        }
    );
    assert_eq!(
        c.swap(c.a, c.g),
        Color {
            r: 255,
            g: 30,
            b: 10,
            a: 200
        }
    );
}