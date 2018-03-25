module EXOR(in1,in2,out);
input in1,in2;
output out;
wire temp1,temp2;
    assign temp1 = in1 & (~in2);
    assign temp2 = in2 & (~in1);
    assign out = temp1 | temp2;
endmodule

module EXOR_TEST;
reg in1,in2;
wire out;
    EXOR exor1 (in1,in2,out);
    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,exor1);

        in1 = 0;
        in2 = 0;
        #100
        in1 = 1;
        in2 = 0;
        #100
        in1 = 0;
        in2 = 1;
        #100
        in1 = 1;
        in2 = 1;
        #200
        $finish;
    end
endmodule
