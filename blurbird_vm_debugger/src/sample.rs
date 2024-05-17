/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-05-14 13:46:42
 * @modify date 2024-05-14 13:46:42
 * @desc [description]
*/


struct VirtualMachine {
    
}

impl VirtualMachine {
    fn execute_op_bad(&mut self, op: u8) {
        if op == 0 {
            //implement add operator
        }
        if op == 1 {
            //implement subtract operator
        }
        if op == 2 {
            //implement multiply operator
        }
        if op == 3 {
            //implement division operator
        }
    }

    fn execute_op_good(&mut self, op: u8) {
        fn add(vm: &mut VirtualMachine, op: u8) {
            //implement add operator
        }
        
        fn sub(vm: &mut VirtualMachine, op: u8) {
            //implement subtract operator
        }
        
        fn mul(vm: &mut VirtualMachine, op: u8) {
            //implement multiply operator
        }
        
        fn div(vm: &mut VirtualMachine, op: u8) {
            //implement division operator
        }

        const HANDLERS: [fn(&mut VirtualMachine, u8) -> (); 4] = [
            add,
            sub,
            mul,
            div
        ];
        let handler = HANDLERS[op as usize];
        handler(self, op);

    }
}

