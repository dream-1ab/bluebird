// module.exports = grammar({
//     name: "bluebird",
//     rules: {
//         source_file: $ => repeat($.struct_definition),
//         struct_definition: $ => seq(
//             "struct",
//             $.identifier,
//             "{",
//             repeat($.struct_field),
//             optional($.last_struct_field),
//             "}"
//         ),
//         identifier: $ => /[a-zA-Z0-9]+/,
//         struct_field: $ => seq(
//             $.identifier,
//             ":",
//             $.data_types,
//             ","
//         ),
//         last_struct_field: $ => seq(
//           $.identifier,
//           ":",
//           $.data_types,
//       ),
//         data_types: $ => choice(
//             "i32",
//             "i64",
//             "f32",
//             "f64",
//             "bool",
//             "string",
//             "char",
//             "u8",
//             "u16",
//             "u32",
//             "u64",
//             "i8",
//             "i16",
//             "i64",
//             "f32",
//             "f64",
//             "bool",
//         ),
//     }
// })


// module.exports = grammar({
//   name: 'my_toy_language',

//   rules: {
//     // The entry point of the grammar
//     source_file: $ => repeat($._definition),

//     // A definition can be a function
//     _definition: $ => $.function_definition,

//     // Define a function
//     function_definition: $ => seq(
//       'fun', 
//       $.identifier, 
//       '(', 
//       ')', 
//       $.block
//     ),

//     // A block is just a pair of braces for now
//     block: $ => seq(
//       '{', 
//       '}', 
//     ),

//     // An identifier (for simplicity, just a sequence of letters)
//     identifier: $ => /[a-zA-Z]+/,
//   }
// });


module.exports = grammar({
  name: "bluebird_asm",
  rules: {
    source_file: $ => repeat(choice(
      $.label,
      $.add_op,
      $.subtract_op,
      $.multiplication_op,
      $.division_op,
      $.modulo_op,
      $.and_op,
      $.or_op,
      $.not_op,
      $.jump_op,
      $.compare,
      $.branch_equals,
      $.branch_not_equals,
      $.branch_greater_than,
      $.branch_less_than,
      $.push_op,
      $.pop_op,
      $.store_op,
      $.call_op,
      $.ret_op,
      $.syscall_op,
    )),

    //common instructions
    decimal_number: $ => /[0-9]+/,
    absolute_memory_address: $ => /\[[0-9]+\]/,
    relative_memory_address: $ => /\[-[0-9]+\]/,
    register: $ => choice(
      "PC", //program counter
      "SP", //stack pointer
      // "CMPR", //comparison register Right side value
      // "CMPL", //comparison register Left side value
    ),
    op_value: $ => choice(
      $.decimal_number,
      $.absolute_memory_address,
      $.relative_memory_address,
      $.register,
      $.label
    ),
    label: $ => /[0-9a-z_]+:/,
    
    //operators
    add_op: $ => seq(
      "ADD",
      $.op_value,
      ",",
      $.op_value,
      ",",
      $.op_value,
    ),
    subtract_op: $ => seq(
      "SUB",
      $.op_value,
      ",",
      $.op_value,
      ",",
      $.op_value,
    ),
    multiplication_op: $ => seq(
      "MUL",
      $.op_value,
      ",",
      $.op_value,
      ",",
      $.op_value,
    ),
    division_op: $ => seq(
      "DIV",
      $.op_value,
      ",",
      $.op_value,
      ",",
      $.op_value,
    ),
    modulo_op: $ => seq(
      "MOD",
      $.op_value,
      ",",
      $.op_value,
      ",",
      $.op_value,
    ),
    and_op: $ => seq(
      "AND",
      $.op_value,
      ",",
      $.op_value,
      ",",
      $.op_value,
    ),
    or_op: $ => seq(
      "OR",
      $.op_value,
      ",",
      $.op_value,
      ",",
      $.op_value,
    ),
    not_op: $ => seq(
      "NOT",
      $.op_value,
      ",",
      $.op_value,
    ),
    jump_op: $ => seq(
      "JMP",
      $.op_value
    ),
    compare: $ => seq(
      "CMP",
      $.op_value,
      ",",
      $.op_value,
    ),
    branch_equals: $ => seq(
      "BEQ",
      $.label,
    ),
    branch_not_equals: $ => seq(
      "BNE",
      $.label,
    ),
    branch_greater_than: $ => seq(
      "BGT",
      $.label,
    ),
    branch_less_than: $ => seq(
      "BLT",
      $.label,
    ),

    //Memory op
    push_op: $ => seq(
      "PUSH",
      $.decimal_number
    ),
    pop_op: $ => seq(
      "POP",
      $.decimal_number
    ),
    store_op: $ => seq(
      "ST",
      $.op_value,
      ",",
      $.op_value,
    ),
    call_op: $ => seq(
      "CALL",
      $.op_value,
    ),
    ret_op: $ => seq("RET"),
    syscall_op: $ => seq("SYSCALL", $.op_value),
  }
})
