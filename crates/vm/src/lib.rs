mod structures;

pub use structures::opcode::*;
pub use structures::value::*;
pub use structures::vm::*;


pub fn test() {
    let mut vm = VM::new();

    vm.globals.push(Value::Integer(1));
    vm.globals.push(Value::Integer(2));

    let code = vec![
        OpCode::Load(0, 0),         // Load global 0 into register 0
        OpCode::Load(1, 1),         // Load global 1 into register 1
        OpCode::Add(0, 0, 1),       // Add registers 0 and 1, store in register 0
        OpCode::Print(0),           // Print register 0
    ];

    let rc_code = std::rc::Rc::new(code);
    let frame = CallFrame::new(rc_code.clone(), 2);
    vm.frames.push(frame);

    println!("{:#?}", vm);
}