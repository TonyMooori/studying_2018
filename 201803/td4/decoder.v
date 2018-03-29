`include "define_opcode.v"

module DECODER(
    input wire [3:0] opcode,
    input wire c_flag,
    output reg [1:0] select,
    output reg [3:0] load
);

    always @(*) begin
        case(opcode)
            `OP_MOV_A_IM:
                {load, select}  <= { 4'b0001, 2'b11 };   
            `OP_MOV_B_IM:
                {load, select}  <= { 4'b0010, 2'b11 };
            `OP_MOV_A_B:
                {load, select}  <= { 4'b0001, 2'b01 };
            `OP_MOV_B_A:
                {load, select}  <= { 4'b0010, 2'b00 };
            `OP_ADD_A_IM:
                {load, select}  <= { 4'b0001, 2'b00 };
            `OP_ADD_B_IM:
                {load, select}  <= { 4'b0010, 2'b01 };
            `OP_IN_A:
                {load, select}  <= { 4'b0001, 2'b10 };
            `OP_IN_B:
                {load, select}  <= { 4'b0010, 2'b10 };
            `OP_OUT_IM:
                {load, select}  <= { 4'b0100, 2'b11 };
            `OP_OUT_B:
                {load, select}  <= { 4'b0100, 2'b01 };
            `OP_JMP_IM:
                {load, select}  <= { 4'b1000, 2'b11 };
            `OP_JNC_IM:
                {load, select}  <= c_flag ?
                    {4'b0000, 2'b00 }:  // don't jump
                    {4'b1000, 2'b11 };  // jump
            default:
                {load, select}  = 6'b000000;
        endcase
    end

endmodule

module DECODER_TEST();
    reg [4:0] opcode_cflag;
    wire [1:0] select;
    wire [3:0] load;
    integer i;
    wire [3:0]opcode;
    wire cflag;

    assign {opcode,cflag} = opcode_cflag;

    DECODER unit1(opcode,c_flag,select,load);

    initial begin
        opcode_cflag = 0;
        for(i=0;i<32;i=i+1) begin
            #100
            // $display("%d,%d,%d,%d,%d | %d,%d,%d,%d,%d,%d",
            //     opcode[3],opcode[2],opcode[1],opcode[0],cflag,
            //     select_b,select_a,load[3],load[2],load[1],load[0]);
            opcode_cflag = opcode_cflag + 1;
        end
        $finish;
    end
endmodule


