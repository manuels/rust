// error-pattern:no public field or method with that name
// xfail-fast
// aux-build:cci_class.rs
use cci_class;
import cci_class::kitties::*;

fn main() {
  let nyan : cat = cat(52u, 99);
  assert (nyan.meows == 52u);
}
