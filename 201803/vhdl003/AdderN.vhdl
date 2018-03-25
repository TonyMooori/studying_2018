library IEEE;
use IEEE.std_logic_1164.all;
use IEEE.std_logic_unsigned.all;    

entity AdderN is
    generic ( N : integer := 8);
    port(
        a,b : in std_logic_vector(N-1 downto 0);
        s : out std_logic_vector(N-1 downto 0);
        c : out std_logic
    );
end;

architecture RTL of Adder4 is
signal add : std_logic_vector(N downto 0);
begin
    add <= ('0' & a) + ('0' & b);
    s <= add(N-1 downto 0);
    c <= add(N);
end;
