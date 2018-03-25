library IEEE;
use IEEE.std_logic_1164.all;

entity HA is
    port(
        a,b : in std_logic;
        s,c : out std_logic
    );
end;

architecture RTL of HA is
signal x,y : std_logic;
begin
    x <= a or b;
    y <= not (a and b);
    s <= x and y;
    c <= not y;
end;
