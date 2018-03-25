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
    reg r1,r2;
    wire out;

    // モジュールEXORのインスタンスunit1
    EXOR unit1 (r1,r2,out);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,unit1);

        r1 = 0; r2 = 0;
        #100
        r1 = 0; r2 = 1;
        #100
        r1 = 1; r2 = 0;
        #100
        r1 = 1; r2 = 1;
        #100
        $finish;
    end
endmodule
