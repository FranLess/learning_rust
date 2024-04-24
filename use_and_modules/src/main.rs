mod foo;
use foo::bar as a;
use foo::foo2::bar2;
// This topic isn't hard to understand,
// we should look at more projects to understand
// modules better though
fn main() {
    a();
    bar2();
}
