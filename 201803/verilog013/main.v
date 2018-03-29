module REG4(
    input [3:0]in,
    input r,
    input clk,
    output [3:0]q
);
    reg [3:0]q;

    always @(posedge r or posedge clk)
        q = r?0:in;
endmodule


module TESTBENCH;
    reg r;
    reg [3:0]d;
    reg clk;
    wire out;
    integer i;

    REG4 u1(d,r,clk,out);

    always #(50)
        clk = ~clk;

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        clk=0;
        d=5;
        
        r=0;
        #100

        r=1;
        #100

        r=0;
        #100

        r=0;
        #100

        $finish;
    end
endmodule
