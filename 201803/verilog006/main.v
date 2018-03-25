module DEC(in,out,err);
    input [3:0] in;
    output [9:0] out;
    output err;

    assign {err,out} = FUNC_DEC(in);

    function[10:0] FUNC_DEC;
        input [3:0]in;

        case(in)
            0: FUNC_DEC = {1'b0,10'b00_0000_0001};
            1: FUNC_DEC = {1'b0,10'b00_0000_0010};
            2: FUNC_DEC = {1'b0,10'b00_0000_0100};
            3: FUNC_DEC = {1'b0,10'b00_0000_1000};
            4: FUNC_DEC = {1'b0,10'b00_0001_0000};
            5: FUNC_DEC = {1'b0,10'b00_0010_0000};
            6: FUNC_DEC = {1'b0,10'b00_0100_0000};
            7: FUNC_DEC = {1'b0,10'b00_1000_0000};
            8: FUNC_DEC = {1'b0,10'b01_0000_0000};
            9: FUNC_DEC = {1'b0,10'b10_0000_0000};
            default: FUNC_DEC = {1'b1,10'b00_0000_0000};
        endcase
    endfunction
endmodule

module TESTBENCH;
    reg [3:0] in;
    wire [9:0] out;
    wire err;
    integer i;

    DEC unit1(in,out,err);

    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,unit1);

        in = 0;
        for(i=0;i<16;i++) begin
            #100
            in += 1;
        end

        $finish;
    end
endmodule
