const fn cmp(x: fn(), y: fn()) -> bool { //~ ERROR function pointers in const fn are unstable
    x == y
}

fn main() {}
