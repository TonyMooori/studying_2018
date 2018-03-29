module DFF(
    input [3:0] d,
    input enable,
    input reset,
    input clk,
    output [3:0] q
);
    reg [3:0] q;

    always @(posedge reset or posedge clk)
        q <= reset ? 0 : (enable ? d : q);
endmodule

