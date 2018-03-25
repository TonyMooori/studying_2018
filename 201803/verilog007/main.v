module tri_buf(in_a,in_b,in_c,en_a,en_b,en_c,out);
    input in_a,in_b,in_c;
    input en_a,en_b,en_c;
    output out;
    tri tri_or;

    bufif1 u1 (tri_or,in_a,en_a);
    bufif1 u2 (tri_or,in_b,en_b);
    bufif1 u3 (tri_or,in_c,en_c);
    buf u4(out,tri_or);
endmodule


module TESTBENCH;
    reg [5:0]in;
    wire out;
    integer i;

    tri_buf u1 (in[0],in[1],in[2],in[3],in[4],in[5],out);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        in = 0;
        for(i=0;i<64;i++) begin
            #100
            in += 1;
        end

        $finish;
    end
endmodule
