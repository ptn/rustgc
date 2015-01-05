// object model: an object that we can allocate contains either an IntVal,
// which is just one integer, or PairVal, which wraps two integers.

enum ObjVal {
  IntVal(int),
  PairVal(int, int)
}

struct Object {
  val: ObjVal,
  marked: bool
}

impl Object {
  fn newInt(val: int) -> Object {
    Object { marked: false, val: ObjVal::IntVal(val) }
  }

  fn newPair(val1: int, val2: int) -> Object {
    Object { marked: false, val: ObjVal::PairVal(val1, val2) }
  }

  fn mark(&mut self) {
    self.marked = true;
  }
}



// virtual machine

static STACK_MAX: uint = 256;

struct VM {
  stack: Vec<Object>,
}

impl VM {
  fn new() -> VM {
    VM { stack: Vec::new() }
  }

  fn size(&self) -> uint {
    self.stack.len()
  }

  fn pop(&mut self, obj: Object) -> Option<Object> {
    self.stack.pop()
  }

  fn push(&mut self, obj: Object) {
    if self.size() < STACK_MAX {
      self.stack.push(obj);
    }
  }

  fn mark(&mut self) {
    for obj in self.stack.iter_mut() {
      obj.mark();
    }
  }
}



fn main() {
}
