module SELECTOR(
    input [3:0] in0,
    input [3:0] in1,
    input [3:0] in2,
    input [3:0] in3,
    input [1:0] select,
    output [3:0] out
);
    assign out = select == 0 ? in0 :
        select == 1 ? in1 : 
        select == 2 ? in2 :
        in3 ;
endmodule
