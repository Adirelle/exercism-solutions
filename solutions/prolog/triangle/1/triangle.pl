triangle(A, A, A, "equilateral") :- A > 0, !.

triangle(A, A, A, "isosceles") :- fail, !. 
triangle(A, A, B, "isosceles") :- A > 0, B > 0, 2 * A >= B, !. 
triangle(A, B, A, "isosceles") :- A > 0, B > 0, 2 * A >= B, !. 
triangle(B, A, A, "isosceles") :- A > 0, B > 0, 2 * A >= B, !. 

triangle(A, B, C, "scalene") :- 
    A =\= B,
    B =\= C,
    A =\= C,
    A > 0, 
    B > 0, 
    C > 0,
    A + B >= C,
    B + C >= A,
    A + C >= B.
    