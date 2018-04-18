module TEST_BENCH();
    reg out;

    initial begin
        $dumpfile("a.vcd");
        $dumpvars(0,out);
        
        #00000000 out=1;
        #00973140 out=0;
        #00001812 out=1;
        #00001808 out=0;
        #00009044 out=1;
        #00001812 out=0;
        #00003616 out=1;
        #00982528 out=0;
        #00001808 out=1;
        #00001808 out=0;
        #00009048 out=1;
        #00001808 out=0;
        #00003620 out=1;
        #00982412 out=0;
        #00001812 out=1;
        #00001808 out=0;

        $finish;
    end
endmodule
