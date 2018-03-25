library IEEE;
use IEEE.std_logic_1164.all;

entity TestBench is
end;

architecture Stimulus of TestBench is 
component HA
    port(
        a,b : in std_logic;
        s,c : out std_logic
    );
end component;

signal aa,bb,ss,cc : std_logic;
begin
    DUT: HA port map(aa,bb,ss,cc);
    aa <= 
        '0',
        '1' after 10 ns,
        '0' after 20 ns,
        '1' after 30 ns,
        '0' after 40 ns,
        '1' after 50 ns,
        '0' after 60 ns;
    BB <=
        '0',
        '1' after 20 ns,
        '0' after 40 ns,
        '1' after 60 ns;
end;

