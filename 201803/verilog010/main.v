module RSFF(r,s,clk,q,q_b);
    input r,s,clk;
    output q,q_b;
    reg q;

    assign q_b = ~q;

    always @(posedge clk)
        case({r,s})
            1: q <= 1;
            2: q <= 0;
            3: q <= 1'bx;
        endcase
endmodule

`timescale 1ns/1ns

module TESTBENCH;
    reg r,s,clk;
    wire out1,out2;
    parameter STEP = 50;

    RSFF u1(r,s,clk,out1,out2);

    always #(STEP/2) clk = ~clk;

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        clk=0;
        r=0;
        s=0;
        #100

        r=1;
        s=0;
        #100

        r=0;
        s=1;
        #100

        r=1;
        s=0;
        #100

        r=1;
        s=1;
        #100

        r=0;
        s=0;
        #100

        r=0;
        s=1;
        #100

        $finish;
    end
endmodule
