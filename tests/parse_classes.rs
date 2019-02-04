extern crate jvm_class_file_parser;

use std::fs::File;
use jvm_class_file_parser::{Bytecode, ClassFile, Code};

#[test]
fn parse_class_dummy() {
    let mut file = File::open("classes/Dummy.class").unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    assert_eq!("Dummy", class_file.get_class_name());

    assert_eq!(Some("Dummy.java"), class_file.get_source_file_name());

    assert_eq!(1, class_file.methods.len());

    let constructor = &class_file.methods[0];
    let code = constructor.get_code(&class_file);

    use Bytecode::*;
    assert_eq!(
        Some(Code {
            max_stack: 1,
            max_locals: 1,
            code: vec![
                (0, Aload_0),
                (1, Invokespecial(1)),
                (4, Return),
            ],
            exception_table: vec![],
        }),
        code
    );
}

#[test]
fn parse_class_intbox() {
    let mut file = File::open("classes/IntBox.class").unwrap();
    let class_file = ClassFile::from_file(&mut file).unwrap();

    assert_eq!("IntBox", class_file.get_class_name());

    assert_eq!(Some("IntBox.java"), class_file.get_source_file_name());

    assert_eq!(2, class_file.methods.len());

    let constructor = &class_file.methods[0];
    let constructor_code = constructor.get_code(&class_file);

    use Bytecode::*;
    assert_eq!(
        Some(Code {
            max_stack: 2,
            max_locals: 2,
            code: vec![
                (0, Aload_0),
                (1, Invokespecial(1)),
                (4, Aload_0),
                (5, Iload_1),
                (6, Putfield(2)),
                (9, Return),
            ],
            exception_table: vec![],
        }),
        constructor_code
    );

    let get_value = &class_file.methods[1];
    let get_value_code = get_value.get_code(&class_file);

    assert_eq!(
        Some(Code {
            max_stack: 1,
            max_locals: 1,
            code: vec![
                (0, Aload_0),
                (1, Getfield(2)),
                (4, Ireturn),
            ],
            exception_table: vec![],
        }),
       get_value_code
    );
}
