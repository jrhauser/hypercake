program Hypercake(input, output);
var
    n, k: integer;



function factorial(n: integer): integer;
var
    value: integer;
begin
    if n <= 1 then Exit(1)
    else 
        value := n * factorial(n-1);
    Exit(value);
end;



function combinations(n, r: integer): integer;
var 
   combination: integer;

begin
    if r <= n then 
       writeln(factorial(n))
       // combination := factorial(n) div (factorial(r) * factorial(n-r))
    else
        combination := 0;
    Exit(combination);
end;




function hypercake(n, k: integer): integer;
var 
    i, pieces : integer;

begin
    pieces := 1;
    if n = 0 then
        Exit(1)
    else
        for i:= 1 to k do 
            begin
                //pieces := pieces + combinations(n, k);
                writeln(combinations(n, k));
            end;
        Exit(pieces);
end;



begin
    writeln('Cuts: ');
    readln(n);
    writeln('Dimensions: ');
    readln(k);
    if (k >= 2) then begin 
        writeln(hypercake(n, k));
    end
    else
        writeln('Not enough dimensions');
end.