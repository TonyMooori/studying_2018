module CPU(
    input reset,
    input clk
);
    wire [3:0]alu_out;
    wire [3:0]load;
    wire [1:0]select;
    wire [3:0]reg_a_out;
    wire [3:0]reg_b_out;
    wire [3:0]reg_c_out;
    wire [3:0]reg_d_out;
    wire [3:0]reg_1_out;
    wire [3:0]alu_in;
    wire [7:0]rom_out;
    wire [3:0]opcode;
    wire [3:0]imm;
    wire carry_out;
    wire [3:0]pc_next;
    
    DFF reg_a(alu_out,load[0],reset,clk,reg_a_out);
    DFF reg_b(alu_out,load[1],reset,clk,reg_b_out);
    DFF reg_c(alu_out,load[2],reset,clk,reg_c_out);
    DFF reg_d(pc_next,   1'b1,reset,clk,reg_d_out);
    DFF reg_1({3'b000,carry_out},1'b1,reset,clk,reg_1_out);

    assign pc_next = load[3] ? alu_out : reg_d_out + 4'b1;

    DECODER decoder(opcode,reg_1_out[0],select,load);

    SELECTOR selector(reg_a_out,reg_b_out,4'b0101,4'b0000,select,alu_in);
    ADDER alu(alu_in,imm,alu_out,carry_out);
    ROM rom(reg_d_out,rom_out);
    
    assign {opcode,imm} = rom_out;
endmodule

module CPU_TEST();
    reg reset;
    reg clk;
    integer i;

    CPU cpu(reset,clk);

    always #100 begin
        clk = ~clk;
        $monitor("a=%04b,b=%04b,out=%04b,pc=%04b"
                                ,cpu.reg_a_out
                                ,cpu.reg_b_out
                                ,cpu.reg_c_out
                                ,cpu.reg_d_out);
    end


    initial begin
        $dumpfile("out.vcd");
        $dumpvars(0,cpu);

        clk = 0;
        reset = 0;
        #1
        reset = 1;
        #1
        reset = 0;

        #3000
        $finish;
    end
endmodule
