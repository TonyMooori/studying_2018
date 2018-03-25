module RSFF(r,s,q,q_b);
    input r,s;
    output q,q_b;
    reg q,q_b;

    //nor u1 (q,r,q_b);
    //nor u2 (q_b,s,q);

    //assign q   = ~(r|q_b);
    // assign q_b = ~(s|q);

    always @(r or s)
        case ({r,s})
            1:begin q <= 1;q_b <= 0;end
            2:begin q <= 0;q_b <= 1;end
            3:begin q <= 0;q_b <= 0;end
        endcase
endmodule

module TESTBENCH;
    reg r,s;
    wire q,q_b;
    integer i,j;

    RSFF u1(r,s,q,q_b);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        r=0;
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

        $finish;
    end
endmodule