// EXORの定義
module EXOR(in1,in2,out);
    input in1,in2;
    output out;

    // function文を用いた定義
    assign out = FUNC(in1,in2);

    function FUNC;
        input a,b;

        if (a^b)
            FUNC = 1;
        else
            FUNC = 0;
    endfunction
endmodule

// テストベンチ
module EXOR_TEST;
    reg [1:0] in;
    wire out;
    integer i;

    EXOR myunit (in[0],in[1],out);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,myunit);

        in = 0;
        for(i=0;i<=4;i=i+1) begin
            #100    in = in + 1;
        end

        $finish;
    end
endmodule