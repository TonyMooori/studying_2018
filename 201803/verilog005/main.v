module SEL(a,b,c,d,sel,out);
    input a,b,c,d;
    input [1:0] sel;
    output out;

    assign out = FUNC(a,b,c,d,sel);

    function FUNC;
        input a,b,c,d;
        input [1:0] sel;

        if( sel == 0)
            FUNC = a;
        else if(sel==1)
            FUNC = b;
        else if(sel==2)
            FUNC = c;
        else
            FUNC = d;
    endfunction
endmodule

module TESTBENCH;
    reg [3:0]in;
    reg [1:0]sel;
    wire out;
    integer i,j;

    SEL u1 (in[0],in[1],in[2],in[3],sel,out);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,u1);

        in = 0;
        for(i=0;i<64;i++) begin
            in += 1;
            sel = 0;
            for(j=0;j<4;j++) begin
                #100
                sel += 1;
            end
        end

        $finish;
    end
endmodule
