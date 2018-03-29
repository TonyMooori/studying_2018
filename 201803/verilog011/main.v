module TFF(
    input r,
    input t,
    input clk,
    output q,
    output q_b
);
    reg q;

    assign q_b = ~q;

    always @(posedge clk or posedge r)
        if(r)
            q <= 0;
        else if(t)
            q <= ~q;
endmodule


module TESTBENCH;
    reg r,t,clk;
    wire out1,out2;

    TFF u1 (r,t,clk,out1,out2);

    
    always #(50) 
        clk = ~clk;

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        clk=0;
        r=0;
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
        
        $finish;
    end
endmodule
