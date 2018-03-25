module RTFF(r,t,q);
    input r,t;
    output q;
    reg q;

    // always @(posedge t)
    //     q <= ~q;

    // always @(posedge r)
    //     q <= 0;

    always @(posedge r or posedge t)
        q <= (r) ? 0 : ~q;
    
endmodule

module TESTBENCH;
    reg r,t;
    wire out;
    
    RTFF u1(r,t,out);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        r=0;
        t=0;
        #100

        r=1;
        t=0;
        #100

        r=0;
        t=1;
        #100

        r=1;
        t=0;
        #100

        r=1;
        t=1;
        #100

        r=0;
        t=0;
        #100

        r=0;
        t=1;
        #100

        $finish;
    end
endmodule