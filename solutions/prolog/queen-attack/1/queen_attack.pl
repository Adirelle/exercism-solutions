create((DimX, DimY)) :-
    DimX >= 0, 
    DimX < 8, 
    DimY >= 0,
    DimY < 8.

attack((FromX, FromY), (ToX, ToY)):-
    create((FromX, FromY)),
    create((ToX, ToY)), 
    (
        FromX =:= ToX;
        FromY =:= ToY;
        abs(FromX - ToX) =:= abs(FromY - ToY)
    ).
