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
  extras: $ => [
    $.comment,
    /\s/
  ],
  rules: {
    source_file: $ => repeat(choice(
      $.label_decleration,
      $._binary_operators,
      $._branch_operators,
      $._memory_operators,
      $._bitwise_operators,
    )),
    comment: $ => choice(
      /;{1}.*/,
      // /(\/\/){1}.+/,
      // /#{1}.+/
    ),
    _data_type: $ => choice(
      "U8",
      "I8",
      "U16",
      "I16",
      "U32",
      "I32",
      "U64",
      "I64",
      "F32",
      "F64",
    ),
    //common instructions
    floating_point_number: $ => /-?\d+\.{1}\d*/,
    negative_integer_number: $ => /-{1}[0-9]+/,
    positive_integer_number: $ => /\+{0,1}[0-9]+/,
    _decimal_number: $ => choice(
      $.negative_integer_number,
      $.positive_integer_number
    ),
    absolute_memory_location: $ => /\[[0-9]+\]/,
    relative_memory_location: $ => /\[-[0-9]+\]/,
    register: $ => choice(
      "PC", //program counter
      "SP", //stack pointer
      // "CMPR", //comparison register Right side value
      // "CMPL", //comparison register Left side value
    ),
    writable_location: $ => choice(
      $.absolute_memory_location,
      $.relative_memory_location,
      $.register,
    ),
    readable_location: $ => choice(
      $._decimal_number,
      $.floating_point_number,
      $.label_definition,
      $.writable_location,
    ),
    label_decleration: $ => /[0-9a-z_]+:/,
    label_definition: $ => /[0-9a-z_]+/,
    _binary_operators: $ => choice(
      $.add_op,
      $.subtract_op,
      $.multiplication_op,
      $.division_op,
      $.modulo_op,
      $.and_op,
      $.or_op,
      $.not_op,
      $.compare_op,
    ),
    _bitwise_operators: $ => choice(
      $.shift_left_op,
      $.shift_right_op,

    ),
    _branch_operators: $ => choice(
      $.jump_op,
      $.branch_equals_op,
      $.branch_not_equals_op,
      $.branch_greater_than_op,
      $.branch_less_than_op,
      $.call_op,
      $.ret_op,
      $.syscall_op,
    ),
    _memory_operators: $ => choice(
      $.push_op,
      $.pop_op,
      $.store_op,
    ),
    //operators
    add_op: $ => seq(
      "ADD",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    subtract_op: $ => seq(
      "SUB",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    multiplication_op: $ => seq(
      "MUL",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    division_op: $ => seq(
      "DIV",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    modulo_op: $ => seq(
      "MOD",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    and_op: $ => seq(
      "AND",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    or_op: $ => seq(
      "OR",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    not_op: $ => seq(
      "NOT",
      $._data_type,
      $.readable_location,
      ",",
      $.writable_location,
    ),
    compare_op: $ => seq(
      "CMP",
      $._data_type,
      $.readable_location,
      ",",
      $.writable_location,
    ),

    shift_left_op: $ => seq(
      "SHL",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),

    shift_right_op: $ => seq(
      "SHR",
      $._data_type,
      field("left", $.readable_location,),
      ",",
      field("right", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),

    jump_op: $ => seq(
      "JMP",
      field("destination", $.readable_location),
    ),
    branch_equals_op: $ => seq(
      "BEQ",
      field("destination", $.readable_location),
    ),
    branch_not_equals_op: $ => seq(
      "BNE",
      field("destination", $.readable_location),
    ),
    branch_greater_than_op: $ => seq(
      "BGT",
      field("destination", $.readable_location),
    ),
    branch_less_than_op: $ => seq(
      "BLT",
      field("destination", $.readable_location),
    ),

    //Memory op
    push_op: $ => seq(
      "PUSH",
      field("size", $._decimal_number)
    ),
    pop_op: $ => seq(
      "POP",
      field("size", $._decimal_number)
    ),
    store_op: $ => seq(
      "ST",
      $._data_type,
      field("value", $.readable_location,),
      ",",
      field("save_to", $.writable_location,)
    ),
    call_op: $ => seq(
      "CALL",
      field("destination", $.readable_location),
    ),
    ret_op: $ => seq("RET"),
    syscall_op: $ => seq("SYSCALL", $.readable_location, $.readable_location, $.readable_location),
  }
})
