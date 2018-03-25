module SEL1(a,b,sel,out);
    input a,b,sel;
    output out;

    assign out = (sel & a) | (~sel & b);
endmodule

module SEL2(a,b,sel,out);
    input a,b,sel;
    output out;
    wire not_sel,sel_a,sel_b;

    not u1 (not_sel,sel);
    and u2 (sel_a,sel,a);
    and u3 (sel_b,sel,b);
    or  u4 (out,sel_a,sel_b);
endmodule

module TEST_BENCH;
    reg [2:0] in;
    wire out1,out2;
    integer i;

    SEL1 selector1(in[0],in[1],in[2],out1);
    SEL2 selector2(in[0],in[1],in[2],out2);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,selector1);
        $dumpvars(1,selector2);

        in = 0;
        for(i=0;i<8;i=i+1) begin
            #100
            in = in + 1;
        end
        $finish;
    end
endmodule
