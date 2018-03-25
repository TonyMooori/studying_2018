library IEEE;
use IEEE.std_logic_1164.all;
use IEEE.std_logic_unsigned.all;    

entity TestBench is
end;

architecture Stimulus of TestBench is 
component AdderN
    generic ( N : integer := 8);
    port(
        a,b : in std_logic_vector(N-1 downto 0);
        s : out std_logic_vector(N-1 downto 0);
        c : out std_logic
    );
end component;

signal aa,bb,ss,cc : std_logic;
begin
    DUT: AdderN port map(aa,bb,ss,cc);
    aa <= 
        "00000001",
        "00000010", after 10 ns,
        "00000011", after 20 ns,
        "00000100", after 30 ns,
        "00000101", after 40 ns;
    bb <=
        "00000001",
        "00000010", after 20 ns,
        "00000011", after 40 ns,
        "00000100", after 60 ns,
        "00000101", after 80 ns;
end;

