`timescale 1ns/1ns

module AND_2_TEST;
reg in1,in2;
wire out;
    AND_2 temp (in1,in2,out);
    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,AND_2_TEST);
        
                in1 = 0;
                in2 = 0;
        #100    in1 = 1;
        #100    in1 = 0;   
                in2 = 1;
        #100    in1 = 1;
        #200    $finish;
    end
endmodule
