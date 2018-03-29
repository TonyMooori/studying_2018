`include "define_opcode.v"

module ROM(
    input[3:0] address,
    output reg [7:0] dout
);
    // JNCテスト
    // 15-> PC=3, 14-> PC=4
    // always @(*)
    //     case (address)
    //     0: dout <= { `OP_MOV_A_IM , 4'b1110 }; 
    //     1: dout <= { `OP_ADD_A_IM , 4'b0001 };
    //     2: dout <= { `OP_JNC_IM   , 4'b0100 };
    //     3: dout <= { `OP_JMP_IM   , 4'b0011 };
    //     4: dout <= { `OP_JMP_IM   , 4'b0100 };
    //     default: dout <= 8'bxxxxxxxx;
    // endcase
    
    // ADD,MOVテスト
    always @(*)
        case (address)
        0: dout <= {`OP_MOV_A_IM , 4'b0000 }; // a = b0000
        1: dout <= {`OP_ADD_A_IM , 4'b1010 }; // a = b1010
        2: dout <= {`OP_MOV_B_IM , 4'b0100 }; // b = b0100
        3: dout <= {`OP_ADD_B_IM , 4'b0001 }; // b = b0101
        4: dout <= {`OP_MOV_A_B  , 4'b0000 }; // a = b0101
        5: dout <= {`OP_JMP_IM   , 4'b0101 };
        default: dout <= 8'bxxxxxxxx;
    endcase
    
endmodule
