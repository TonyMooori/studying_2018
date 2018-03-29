module ADDER(
    input [3:0] a,
    input [3:0] b,
    output [3:0] out,
    output carry_out
);
    assign {carry_out,out} = a + b;
endmodule
