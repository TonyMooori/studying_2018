module DFF(
    input in,
    input r,
    input clk,
    output q
);
    reg q;

    always @(posedge r or posedge clk)
        q = r?0:in;
endmodule


module TESTBENCH;
    reg [1:0]in;
    reg clk;
    wire out;
    integer i;

    DFF u1(in[0],in[1],clk,out);

    always #(50)
        clk = ~clk;

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        clk=0;
        in = 0;
        for(i=0;i<8;i=i+1) begin
            #100
            in = in + 1;
        end

        $finish;
    end
endmodule
