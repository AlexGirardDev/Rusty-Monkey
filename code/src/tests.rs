use crate::opcode::Opcode;

struct Test {
    opcode: Opcode,
    opperands: Vec<usize>,
    expected: Vec<u8>,
}

impl Test {
    fn new(opcode: Opcode, opperands: Vec<usize>, expected: Vec<u8>) -> Self {
        Self {
            opcode,
            opperands,
            expected,
        }
    }
}

#[test]
fn test_make() {
    let tests = vec![
        Test::new( Opcode::Constant, vec![65534], vec![Opcode::Constant as u8, 255, 254]),
        Test::new( Opcode::Add, vec![], vec![Opcode::Add as u8]),
        Test::new( Opcode::Pop, vec![], vec![Opcode::Pop as u8])
    ];
    for Test {
        opcode,
        opperands,
        expected,
    } in tests
    {
        let instructions = opcode.make_with(&opperands);
        assert_eq!(
            instructions.len(),
            expected.len(),
            "insrtuction leng was wrong"
        );
        for (i, b) in expected.iter().enumerate() {
            assert_eq!(instructions[i], *b, "wrong bytes at pos {i}");
        }
    }
}
